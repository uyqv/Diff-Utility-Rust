// use rayon::prelude::*;
use std::str::from_utf8;

#[derive(Debug, PartialEq)]
pub struct Difference<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

pub struct DiffEngine {}

impl DiffEngine {
    pub fn new() -> DiffEngine {
        DiffEngine {}
    }

    pub fn compare_chunks<'a>(&self, chunk1: &'a [u8], chunk2: &'a [u8]) -> Vec<Difference<'a>> {
        let content1 = from_utf8(chunk1).unwrap_or_default();
        let content2 = from_utf8(chunk2).unwrap_or_default();
        let min_length = content1.len().min(content2.len());
        content1.lines()
            .zip(content2.lines())
            .enumerate()
            .filter_map(|(index, (line1, line2))| {
                if index < min_length && line1 != line2 {
                    Some(Difference { from: line1, to: line2 })
                } else {
                    None
                }
            })
            .collect()
    }
}
