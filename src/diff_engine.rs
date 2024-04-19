pub struct DiffEngine {
    line_number_offset: usize, // tracks cumulative line numbers across chunks
}

#[derive(Debug, PartialEq)] // automatically implements PartialEq for Difference
pub struct Difference {
    pub line_number: usize,
    pub from: String,
    pub to: String,
}

impl DiffEngine {
    // Constructor for DiffEngine
    // RETURNS: an instance of DiffEngine
    pub fn new() -> DiffEngine {
        DiffEngine {
            line_number_offset: 0, // initialize with zero offset
        }
    }

    // Compares two chunks and returns a list of differences
    // PARAMETERS: two chunks of data
    // RETURNS: a list of Differences 
    pub fn compare_chunks(&mut self, chunk1: &[u8], chunk2: &[u8]) -> Vec<Difference> {
        // Convert byte slices to UTF-8 strings
        let content1 = std::str::from_utf8(chunk1).unwrap_or_default();
        let content2 = std::str::from_utf8(chunk2).unwrap_or_default();

        // Split the strings into lines or units for comparison
        let lines1: Vec<&str> = content1.lines().collect();
        let lines2: Vec<&str> = content2.lines().collect();

        let lines1_len = lines1.len();

        // Calculate the differences for the current chunk
        let differences: Vec<Difference> = lines1
            .into_iter()
            .zip(lines2.into_iter())
            .enumerate()
            .filter_map(|(index, (line1, line2))| {
                if line1 != line2 {
                    Some(Difference {
                        line_number: self.line_number_offset + index + 1, // calculate absolute line number
                        from: line1.to_string(),
                        to: line2.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        // Update the line number offset for the next chunk
        self.line_number_offset += lines1_len; // increase by number of lines processed in this chunk
        
        differences
    }
}
