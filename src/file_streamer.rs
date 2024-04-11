use std::fs::File; // file operations
use std::io::{self, BufRead, BufReader, Seek, SeekFrom}; //IO operations
use std::sync::mpsc; // inter-thread communication
use std::thread; // concurrent communication

// PARAMETERS: file path
// RETURNS: a "Result" with two vectors of strings, one for each half of the file 
pub fn parallel_read(file_path: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len(); // size of the file in bytes

    // divides the file reading task
    let half = file_size / 2;
    // tx is the transmitter, rx is the receiver
    let (tx, rx) = mpsc::channel();
    // clones the transmitter so can be used in multiple threads
    let tx1 = mpsc::Sender::clone(&tx);

    // necessary to clone file path due to ownership of file_path varaible for each thread
    let file_path_clone = file_path.to_string();

    // reading first half in a thread and spawns a new thread
    thread::spawn(move || { //"move" used to transfer ownership of captured variables
        let file = File::open(&file_path_clone).unwrap();
        let mut reader = BufReader::new(file); //wraps the file for efficient reading
        let mut lines = Vec::new(); // hold the lines read
        let mut buffer = String::new(); // hold the current line
        while reader.stream_position().unwrap() < half {
            reader.read_line(&mut buffer).unwrap();
            lines.push(buffer.clone());
            buffer.clear();
        }
        tx1.send(lines).unwrap(); // sends the collected lines through channel "tx1" to the main thread
    });

    // reading second half in a thread and spawns a new thread
    let file_path_clone = file_path.to_string();
    thread::spawn(move || {
        let file = File::open(&file_path_clone).unwrap();
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(half)).unwrap(); // move the file pointer to the middle of the file
        let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap(); // collects all remaining line into a vector
        tx.send(lines).unwrap(); 
    });

    // main thread waits for both threads to finish and recieves vectors containing the lines
    let first_half = rx.recv().unwrap(); 
    let second_half = rx.recv().unwrap();

    Ok((first_half, second_half))
}
