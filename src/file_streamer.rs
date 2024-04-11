use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::sync::mpsc;
use std::thread;

pub fn parallel_read(file_path: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let half = file_size / 2;
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    let file_path_clone = file_path.to_string();
    thread::spawn(move || {
        let file = File::open(&file_path_clone).unwrap();
        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut buffer = String::new();
        while reader.stream_position().unwrap() < half {
            reader.read_line(&mut buffer).unwrap();
            lines.push(buffer.clone());
            buffer.clear();
        }
        tx1.send(lines).unwrap();
    });

    let file_path_clone = file_path.to_string();
    thread::spawn(move || {
        let file = File::open(&file_path_clone).unwrap();
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(half)).unwrap();
        let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
        tx.send(lines).unwrap();
    });

    let first_half = rx.recv().unwrap();
    let second_half = rx.recv().unwrap();

    Ok((first_half, second_half))
}
