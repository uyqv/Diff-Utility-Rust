mod file_streamer;
mod diff_engine;
mod cli;

use file_streamer::FileStreamer;
use diff_engine::DiffEngine;
use std::time::Instant;

fn main() {
    let (file1, file2) = cli::parse_arguments();
    // let file1 = String::from("src/data/temp1.txt");
    // let file2 = String::from("src/data/temp2.txt");
    if let Err(e) = run_diff(&file1, &file2) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// PARAMETERS: file paths
// RETURNS: error message if panic occurs
fn run_diff(file1: &str, file2: &str) -> Result<(), String> {
    println!("Comparing files {} and {}", file1, file2);
    let chunk_size = 1024 * 1024; // 1 MB, will become configurable later

    let start_time = Instant::now();
    
    // creates a new instance of FileStreamer
    let file_streamer = FileStreamer::new(file1, file2, chunk_size);

    let file_read_time = start_time.elapsed();

    // create a new instance of DiffEngine
    let diff_engine = DiffEngine::new();

    // creates a stream of chunks from both files
    match file_streamer.chunk_stream() {
        Ok(chunk_stream) => { //checks if stream is Ok
            for chunk_pair in chunk_stream { // iterates over each pair of chunks
                match chunk_pair {
                    Ok((chunk1, chunk2)) => {
                        let differences = diff_engine.compare_chunks(&chunk1, &chunk2); // calculates the difference
                        if !differences.is_empty() {
                            println!("\n{:?}", differences);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading chunks: {}", e);
                        break;
                    }
                }
            }

            let diff_time = start_time.elapsed() - file_read_time;

            println!("\nFile read time: {:?}", file_read_time);
            println!("Diff computation time: {:?}", diff_time);
            println!("Total time: {:?}", start_time.elapsed());
        }
        Err(e) => eprintln!("Error creating chunk stream: {}", e),
    }

    Ok(())
}

