use std::fs;

pub fn read_csv_to_string(filename: &str) {
    let file_string = fs::read_to_string(filename).expect("Unable to read file");
    println!("{:?}", file_string);
}
