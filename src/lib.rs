use std::fs::File;
use std::io::{self, Read};

pub fn compare_files(file_path1: &str, file_path2: &str) -> io::Result<bool> {
    let mut file1 = File::open(file_path1)?;
    let mut file2 = File::open(file_path2)?;

    let mut buffer1 = Vec::new();
    let mut buffer2 = Vec::new();

    file1.read_to_end(&mut buffer1)?;
    file2.read_to_end(&mut buffer2)?;

    Ok(buffer1 == buffer2)
}
