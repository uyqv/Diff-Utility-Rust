use std::time::Instant;

mod cli;
mod file_streamer;
mod diff_engine;

fn main() {
    let (file1, file2) = cli::parse_arguments();

    if let Err(e) = run_diff(&file1, &file2) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_diff(file1: &str, file2: &str) -> Result<(), String> {
    println!("Comparing files {} and {}", file1, file2);

    let start_time = Instant::now();

    let (content1_first_half, content1_second_half) = file_streamer::parallel_read(file1)
        .map_err(|e| format!("Failed to read {}: {}", file1, e))?;

    let (content2_first_half, content2_second_half) = file_streamer::parallel_read(file2)
        .map_err(|e| format!("Failed to read {}: {}", file2, e))?;

    let file_read_time = start_time.elapsed();

    let _diffs = diff_engine::compute_diff(
        content1_first_half, content1_second_half, 
        content2_first_half, content2_second_half)
        .map_err(|e| format!("Failed to compute differences: {}", e))?;

    let diff_time = start_time.elapsed() - file_read_time;

    // if diffs.is_empty() {
    //     println!("No differences found.");
    // } else {
    //     println!("Differences found:");
    //     for diff in diffs {
    //         println!("{}", diff);
    //     }
    // }

    println!("File read time: {:?}", file_read_time);
    println!("Diff computation time: {:?}", diff_time);
    println!("Total time: {:?}", start_time.elapsed());

    // let start_time1 = Instant::now();

    // let content1 = file_streamer1::stream_file(file1)
    // .map_err(|e| e.to_string())?;
    // let content2 = file_streamer1::stream_file(file2)
    // .map_err(|e| e.to_string())?;

    // let file_read_time1 = start_time1.elapsed();

    // let _diff = diff_engine1::compute_diff(content1, content2)
    // .map_err(|e| e.to_string())?;

    // let diff_time1 = start_time1.elapsed() - file_read_time1;
    // println!("File read time: {:?}", file_read_time1);
    // println!("Diff computation time: {:?}", diff_time1);
    // println!("Total time: {:?}", start_time1.elapsed());

    Ok(())
}
