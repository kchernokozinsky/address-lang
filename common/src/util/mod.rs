use std::fs;

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(&path).expect("Should have been able to read the file");
    return contents;
}
