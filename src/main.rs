mod file_streamer;
mod diff_engine;
mod cli;

use file_streamer::FileStreamer;
use diff_engine::DiffEngine;
use std::time::{Instant, Duration};
use std::fs::OpenOptions;
use std::io::Write;
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
fn run_diff(file1: &str, file2: &str, chunk_size: usize, max_mismatches_before_stop: usize) -> Result<(), String> {
    let file_pair = format!("{} vs {}", file1, file2);
    let start_time = Instant::now();
    let mut diff_count = 0;

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
                            println!("\n{:?}", differences);
                            diff_count += 1;
                            if diff_count >= max_mismatches_before_stop {
                                break;
                            }
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
            writeln!(log_file) // Ensures JSON objects are on new lines
                .map_err(|e| e.to_string())?;

        }
        Err(e) => eprintln!("Error creating chunk stream: {}", e),
    }

    Ok(())
}

