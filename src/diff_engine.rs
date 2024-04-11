// In diff_engine.rs

pub fn compute_diff(
    content1_first_half: Vec<String>,
    content1_second_half: Vec<String>,
    content2_first_half: Vec<String>,
    content2_second_half: Vec<String>,
) -> Result<Vec<String>, String> {
    let mut differences = Vec::new();

    let content1 = [content1_first_half, content1_second_half].concat();
    let content2 = [content2_first_half, content2_second_half].concat();

    for (line1, line2) in content1.iter().zip(content2.iter()) {
        if line1 != line2 {
            differences.push(format!("File1: '{}', File2: '{}'", line1, line2));
        }
    }

    Ok(differences)
}
