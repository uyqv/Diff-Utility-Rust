use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc;
use std::thread;

pub fn parallel_read(file_path: &str) -> io::Result<(Vec<String>, Vec<String>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let (tx, rx) = mpsc::channel();

    let half = lines.len() / 2;

    let lines_first_half = lines[..half].to_vec();
    let lines_second_half = lines[half..].to_vec();

    let tx1 = mpsc::Sender::clone(&tx);
    let handle1 = thread::spawn(move || {
        tx1.send(lines_first_half).unwrap();
    });

    let handle2 = thread::spawn(move || {
        let second_half_reversed = lines_second_half.into_iter().rev().collect();
        tx.send(second_half_reversed).unwrap();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let first_half = rx.recv().unwrap();
    let second_half = rx.recv().unwrap();

    Ok((first_half, second_half))
}
