use clap::{App, Arg};

pub fn parse_arguments() -> (String, String, usize) {
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
        .arg(Arg::with_name("chunk_size")
            .help("The chunk size for reading files")
            .takes_value(true) // Indicates this argument takes a value
            .required(false)
            .default_value("1048576") // Default chunk size of 1 MB
            .index(3))
        .get_matches();

    let file1 = matches.value_of("file1").unwrap().trim().to_string();
    let file2 = matches.value_of("file2").unwrap().trim().to_string();
    let chunk_size = matches.value_of_t::<usize>("chunk_size").unwrap_or_else(|_| 1024 * 1024); // Using 1 MB default value

    (file1, file2, chunk_size)
}
