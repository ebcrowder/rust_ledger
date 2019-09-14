mod read_csv;

use std::env;

fn main() {
    // collect args into a Vector and assign them to vars
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // read contents of CSV and print
    read_csv::read_csv_to_string(filename);
}
