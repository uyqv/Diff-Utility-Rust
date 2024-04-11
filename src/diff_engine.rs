use std::sync::mpsc;
use std::thread;

pub fn compute_diff(
    content1_first_half: Vec<String>,
    content1_second_half: Vec<String>,
    content2_first_half: Vec<String>,
    content2_second_half: Vec<String>,
) -> Result<Vec<String>, String> {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    let handler1 = thread::spawn(move || {
        for (line1, line2) in content1_first_half.iter().zip(content2_first_half.iter()) {
            if line1 != line2 {
                tx.send(format!("File1: '{}', File2: '{}'", line1, line2)).unwrap();
            }
        }
    });

    let handler2 = thread::spawn(move || {
        for (line1, line2) in content1_second_half.iter().zip(content2_second_half.iter()) {
            if line1 != line2 {
                tx1.send(format!("File1: '{}', File2: '{}'", line1, line2)).unwrap();
            }
        }
    });

    handler1.join().unwrap();
    handler2.join().unwrap();

    let differences = rx.iter().collect::<Vec<_>>();

    Ok(differences)
}
