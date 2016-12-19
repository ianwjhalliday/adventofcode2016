use std::fs::File;
use std::io::Read;

pub fn get_input(file_path: &'static str) -> String {
    let mut file = File::open(file_path).expect("Failed to open input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Failed to read input file into String");

    input
}
