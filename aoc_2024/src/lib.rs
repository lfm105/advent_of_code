use std::{fs::File, io::{BufReader, BufRead}};

pub fn get_input_file() -> File
{
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("File name not provided. Usage: cargo run --bin <day>_<part> <input_file>");
    } else {
        File::open(&args[1]).unwrap_or_else(|error| {
            panic!("Could not open file {}. Error: {}", args[1], error);
        })
    }
}

pub fn read_file_lines(file: File) -> Vec<String>
{
    BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect()
}
