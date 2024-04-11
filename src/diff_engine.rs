use rayon::prelude::*; // enables data-parallel operations

pub struct DiffEngine {}

#[derive(Debug, PartialEq)] // automatically implements PartialEq for Difference
pub struct Difference {
    pub from: String,
    pub to: String,
}

// constructor for DiffEngine
// RETURNS: an instance of DiffEngine
impl DiffEngine {
    pub fn new() -> DiffEngine {
        DiffEngine {}
    }

    // compares two chunks and returns a list of differences
    // PARAMETERS: two chunks of data
    // RETURNS: a list of Differences 
    pub fn compare_chunks(&self, chunk1: &[u8], chunk2: &[u8]) -> Vec<Difference> {
        // convert byte slices to UTF-8 strings
        let content1 = std::str::from_utf8(chunk1).unwrap_or_default();
        let content2 = std::str::from_utf8(chunk2).unwrap_or_default();

        // split the strings into lines or units for comparison
        let lines1: Vec<&str> = content1.lines().collect();
        let lines2: Vec<&str> = content2.lines().collect();

        // find the minimum length to avoid index out of bounds
        let min_length = lines1.len().min(lines2.len());

        // compare each line in parallel
        lines1
            .into_par_iter() 
            .zip(lines2.into_par_iter()) // pairs elements together, preparing for parallel processing
            .enumerate() // tranforms the iterator of pairs into an iterator of tuples
            .filter_map(|(index, (line1, line2))| {
                if index < min_length && line1 != line2 {
                    Some(Difference {
                        from: line1.to_string(),
                        to: line2.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect() // collects result from filter_map that was flagged by "Some(..)"
    }
}
