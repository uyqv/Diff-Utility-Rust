use clap::{App, Arg};
use regex::Regex;

pub fn parse_arguments() -> (String, String, usize, usize, Option<Regex>) {
    let matches = App::new("DiffUtility")
        .version("1.0")
        .author("Andrew Shatsky")
        .about("Compares two files to find differences")
        .arg(Arg::new("file1")
            .help("The first file to compare")
            .required(true)
            .index(1))
        .arg(Arg::new("file2")
            .help("The second file to compare")
            .required(true)
            .index(2))
        .arg(Arg::new("chunk_size")
            .help("The chunk size for reading files")
            .takes_value(true)
            .required(false)
            .default_value("1048576")
            .long("chunk-size"))  
        .arg(Arg::new("max_mismatches_before_stop")
            .help("Maximum number of mismatches before stopping comparison")
            .takes_value(true)
            .required(false)
            .default_value("1000")
            .long("max-mismatches-before-stop"))
        .arg(Arg::new("ignore_pattern")
            .help("Regex pattern to ignore at the start of lines")
            .takes_value(true)
            .required(false)
            .long("ignore-pattern"))
        .get_matches(); 

    let file1 = matches.value_of("file1").unwrap().trim().to_string();
    let file2 = matches.value_of("file2").unwrap().trim().to_string();
    let chunk_size = matches.value_of_t::<usize>("chunk_size").unwrap_or_else(|_| 1024 * 1024);
    let max_mismatches_before_stop = matches.value_of_t::<usize>("max_mismatches_before_stop").unwrap_or_else(|_| 100);
    let ignore_pattern = matches.value_of("ignore_pattern").map(|p| Regex::new(p).expect("Invalid regex pattern"));


    (file1, file2, chunk_size, max_mismatches_before_stop, ignore_pattern)
}
