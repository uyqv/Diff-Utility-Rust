use clap::{App, Arg};

pub fn parse_arguments() -> (String, String) {
    let matches = App::new("DiffUtility")
        .version("1.0")
        .author("Andrew Shatsky")
        .about("Compares two files to find differences")
        .arg(Arg::with_name("file1")
            .help("The first file to compare")
            .required(true)
            .index(1)) 
        .arg(Arg::with_name("file2")
            .help("The second file to compare")
            .required(true)
            .index(2)) 
        .get_matches();
    
    let file1 = matches.value_of("file1").unwrap();
    let file2 = matches.value_of("file2").unwrap();
    let file1_trimmed = file1.trim().to_string();
    let file2_trimmed = file2.trim().to_string();

    (file1_trimmed, file2_trimmed)
}
