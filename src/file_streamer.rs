use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn stream_file(file_path: &str) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    Ok(reader.lines())
}
