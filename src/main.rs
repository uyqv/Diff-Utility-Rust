mod cli;
mod file_streamer;
mod diff_engine;


fn main() {
    // 
    let (file1, file2) = cli::parse_arguments();

    if let Err(e) = run_diff(&file1, &file2) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}


fn run_diff(file1: &str, file2: &str) -> Result<(), String> {
    println!("Comparing files {} and {}", file1, file2);

    let content1 = file_streamer::stream_file(file1)
    .map_err(|e| e.to_string())?;
    let content2 = file_streamer::stream_file(file2)
    .map_err(|e| e.to_string())?;

    let diff = diff_engine::compute_diff(content1, content2)
    .map_err(|e| e.to_string())?;

    println!("Diff: \n{}", diff);

    Ok(())
}
