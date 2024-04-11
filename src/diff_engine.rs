use rayon::prelude::*; // parallel processing

// PARAMETERS: four vector of strings
// RETURNS: a vector of strings containing the differences in 'Result' type
pub fn compute_diff(
    content1_first_half: Vec<String>,
    content1_second_half: Vec<String>,
    content2_first_half: Vec<String>,
    content2_second_half: Vec<String>,
) -> Result<Vec<String>, String> {
    // chains the two vectors together and returns a Iterator
    let content1_iter = content1_first_half.into_iter().chain(content1_second_half.into_iter());
    let content2_iter = content2_first_half.into_iter().chain(content2_second_half.into_iter());

    // Use par_bridge to turn the chained iterator into a parallel iterator
    let differences: Vec<_> = content1_iter
        .zip(content2_iter) // pairs elements together, preparing for parallel processing
        .par_bridge() // Convert to parallel iterator
        .filter_map(|(line1, line2)| {
            if line1 != line2 {
                Some(format!("File1: '{}', File2: '{}'", line1, line2))
            } else {
                None
            }
        })
        .collect(); // collects result from filter_map that was flagged by "Some(..)"

    Ok(differences)
}
