// returns balances of all general ledger accounts

// use regex::Regex;
use std::fs;

pub fn balance(filename: &str) -> Result<(), std::io::Error> {
    let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");
    let account_vec: Vec<&str> = file_string.split_ascii_whitespace().collect();

    println!("{:?}", account_vec);

    Ok(())
}
