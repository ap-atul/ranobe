use std::fs;

pub fn write_txt_to_file(filename: &str, text: String) {
    let write_err = format!("Failed writing to file : {}", filename);
    fs::write(filename, text).expect(&write_err)
}

pub fn read_txt_from_file(filename: &str) -> String {
    let read_err = format!("Failed reading from file : {}", filename);
    fs::read_to_string(filename).expect(&read_err)
}
