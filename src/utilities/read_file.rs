use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Couldn't read file")
}
