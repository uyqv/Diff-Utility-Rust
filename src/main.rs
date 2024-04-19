mod file_streamer;
mod diff_engine;
mod cli;

use file_streamer::FileStreamer;
use diff_engine::DiffEngine;
use std::time::{Instant, Duration};
use diff_engine::Difference;
use std::fs::OpenOptions;
use std::io::Write;
use colored::*;
use serde_json::json;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct LogData {
    start_time: String,
    total_time: Duration,
}

fn main() {
    let (
        file1, 
        file2, 
        chunk_size,
        max_mismatches_before_stop,
        _ignore_pattern
    ) = cli::parse_arguments();

    // let file1 = String::from("src/data/temp1.txt");
    // let file2 = String::from("src/data/temp2.txt");
    if let Err(e) = run_diff(&file1, &file2, chunk_size, max_mismatches_before_stop) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// PARAMETERS: file paths
// RETURNS: error message if panic occurs
fn run_diff(file1: &str, file2: &str, chunk_size: usize, _max_mismatches_before_stop: usize) -> Result<(), String> {
    let file_pair = format!("{} vs {}", file1, file2);
    let start_time = Instant::now();

    // creates a new instance of FileStreamer
    let file_streamer = FileStreamer::new(file1, file2, chunk_size);
    // create a new instance of DiffEngine
    let mut diff_engine = DiffEngine::new();

    // creates a stream of chunks from both files
    match file_streamer.chunk_stream() {
        Ok(chunk_stream) => { //checks if stream is Ok
            for chunk_pair in chunk_stream { // iterates over each pair of chunks
                match chunk_pair {
                    Ok((chunk1, chunk2)) => {
                        let differences = diff_engine.compare_chunks(&chunk1, &chunk2); // calculates the difference
                        if !differences.is_empty() {
                            print_difference(&differences, &chunk1, &chunk2);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading chunks: {}", e);
                        break;
                    }
                }
            }
            let total_time = start_time.elapsed();
            println!("\nTotal time: {:?}", total_time);

            // store time results into json file
            let log_data = LogData {
                start_time: format!("{:?}", start_time),
                total_time,
            };
            let log_entry = json!({ file_pair: log_data });
            let mut log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log_results.json")
                .map_err(|e| e.to_string())?;
            serde_json::to_writer(&mut log_file, &log_entry)
                .map_err(|e| e.to_string())?;
            writeln!(log_file) 
                .map_err(|e| e.to_string())?;

        }
        Err(e) => eprintln!("Error creating chunk stream: {}", e),
    }

    Ok(())
}

// fn print_difference(differences: &[Difference]) {
//     for diff in differences {
//         println!(
//             "{}: {}, \"{}\" → \"{}\"",
//             "Line".bright_blue(),
//             diff.line_number.to_string().bright_blue(),
//             diff.from.red(),
//             diff.to.green()
//         );
//     }
// }

fn print_difference(differences: &[Difference], chunk1: &Vec<u8>, chunk2: &Vec<u8>) {
    let content1 = std::str::from_utf8(chunk1).unwrap_or_default();
    let content2 = std::str::from_utf8(chunk2).unwrap_or_default();

    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();

    for diff in differences {
        let line_number = diff.line_number;
        let context_range1 = std::cmp::max(0, line_number as isize - 3) as usize..std::cmp::min(lines1.len(), line_number + 2);
        let context_range2 = std::cmp::max(0, line_number as isize - 3) as usize..std::cmp::min(lines2.len(), line_number + 2);

        println!("\nContext for Line {}:", line_number.to_string().yellow());
        
        println!("{}:", "File 1".bright_blue());
        for index in context_range1.clone() {
            if index < lines1.len() {
                println!("{} {}: {}", "Line".bright_blue(), index + 1, lines1[index]);
            }
        }

        println!("{}:", "File 2".bright_blue());
        for index in context_range2.clone() {
            if index < lines2.len() {
                println!("{} {}: {}", "Line".bright_blue(), index + 1, lines2[index]);
            }
        }

        println!(
            "{}: {}, \"{}\" → \"{}\"",
            "Diff".bright_purple(),
            diff.line_number.to_string().bright_purple(),
            diff.from.red(),
            diff.to.green()
        );
    }
}
