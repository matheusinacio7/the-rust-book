use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_pattern = args.get(1).expect("Must inform a search pattern");
    let file_path = args.get(2).expect("Must inform a file path");

    let file_contents = fs::read_to_string(file_path).expect("Could not read file");
    for (i, line) in file_contents.lines().enumerate() {
        println!("Line {}: {}", i, line);
    }
}
