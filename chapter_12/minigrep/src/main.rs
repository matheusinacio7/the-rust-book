use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_pattern = args.get(1).expect("Must inform a search pattern");
    let file_path = args.get(2).expect("Must inform a file path");
}
