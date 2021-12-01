use std::fs;

pub fn get_data(filename: &str) -> String {
    fs::read_to_string(format!("inputs/{}", filename)).unwrap()
}
