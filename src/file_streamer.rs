use std::fs::File;
use std::io::{self, BufReader, Read};

// hold file paths and chunk size
pub struct FileStreamer {
    file_path1: String,
    file_path2: String,
    chunk_size: usize,
}

// defines two methods, one for creating instances of FileStreamer and one for reading chunks
impl FileStreamer {
    pub fn new(file_path1: &str, file_path2: &str, chunk_size: usize) -> FileStreamer {
        FileStreamer {
            file_path1: file_path1.to_string(),
            file_path2: file_path2.to_string(),
            chunk_size: chunk_size,
        }
    }

    // creates an iterator for reading chunks from both files
    pub fn chunk_stream(&self) -> io::Result<ChunkStream> {
        let file1 = File::open(&self.file_path1)?;
        let file2 = File::open(&self.file_path2)?;

        Ok(ChunkStream {
            reader1: BufReader::new(file1),
            reader2: BufReader::new(file2),
            chunk_size: self.chunk_size,
        })
    }
}

pub struct ChunkStream {
    reader1: BufReader<File>,
    reader2: BufReader<File>,
    chunk_size: usize,
}

impl Iterator for ChunkStream {
    type Item = io::Result<(Vec<u8>, Vec<u8>)>; 

    // defines how the iterator fetches the chunk of data from each file
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf1 = vec![0; self.chunk_size];
        let mut buf2 = vec![0; self.chunk_size];

        match (self.reader1.read(&mut buf1), self.reader2.read(&mut buf2)) {
            (Ok(bytes_read1), Ok(bytes_read2)) => {
                if bytes_read1 == 0 && bytes_read2 == 0 {
                    return None; // End of both files
                }
                buf1.truncate(bytes_read1); // Trim buffer to actual data size
                buf2.truncate(bytes_read2); 
                Some(Ok((buf1, buf2)))
            },
            (Err(e), _) | (_, Err(e)) => Some(Err(e)),
        }
    }
}