use std::env;
use std::fs;

fn main() {
    // print args
    for argument in env::args() {
        println!("{}", argument);
    }
    // read contents of CSV and print
    read_csv();
}

fn read_csv() {
    let file_string = fs::read_to_string("test.csv").expect("Unable to read file");
    println!("{:?}", file_string);
}
