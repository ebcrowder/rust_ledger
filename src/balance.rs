// returns balances of all general ledger accounts

use regex::Regex;
use std::fs;

pub fn balance(filename: &str) -> Result<(), std::io::Error> {
    let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");
    let account_vec: Vec<&str> = file_string.split('\n').collect();

    let mut results = Vec::new();
    let date_regex = Regex::new(r"\d{4}/\d{2}/\d{2}").unwrap();

    for line in account_vec {
        if date_regex.is_match(line) {
            results.push(line);
        }
    }

    println!("{:?}", results);

    Ok(())
}
