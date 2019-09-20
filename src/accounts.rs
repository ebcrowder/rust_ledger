// accounts returns all general ledger accounts utilized in .dat ledger file

use std::fs;

pub fn read_csv_to_string(filename: &str) {
    let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");

    // split words by separate lines
    let mut word_collection: Vec<&str> = file_string.split_ascii_whitespace().collect();
    word_collection.sort();
    word_collection.dedup();

    for word in word_collection {
        if word.contains(':') {
            println!("{}", word);
        }
    }
}
