pub fn compute_diff<I1, I2>(file1_lines: I1, file2_lines: I2) -> Result<String, std::io::Error>
where
    I1: Iterator<Item = Result<String, std::io::Error>>,
    I2: Iterator<Item = Result<String, std::io::Error>>,
{
    let mut diff_result = String::new();

    
    let mut file1_iter = file1_lines.peekable();
    let mut file2_iter = file2_lines.peekable();

    while file1_iter.peek().is_some() || file2_iter.peek().is_some() {
        match (file1_iter.next(), file2_iter.next()) {
            (Some(Ok(line1)), Some(Ok(line2))) => {
                if line1 != line2 {
                    diff_result.push_str(&format!("- {}\n+ {}\n", line1, line2));
                }
            },
            (Some(Ok(line)), None) => diff_result.push_str(&format!("- {}\n", line)),
            (None, Some(Ok(line))) => diff_result.push_str(&format!("+ {}\n", line)),
            (Some(Err(e)), _) | (_, Some(Err(e))) => return Err(e),
            (None, None) => break,
        }
    }

    Ok(diff_result)
}
