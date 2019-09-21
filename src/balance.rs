// returns balances of all general ledger accounts

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn balance(filename: &str) -> io::Result<()> {
    // edit this eventually
    #[allow(dead_code)]
    struct AccountSum {
        account: String,
        balance: i32,
    }

    let file = File::open(filename).unwrap(); // fail if the file doesn't exist
    let reader = BufReader::new(file);
    let accounts: Vec<String> = reader
        .lines()
        .filter_map(|line_result| line_result)
        // .filter_map(|line| line.parse().ok())
        .collect();

    println!("accounts {:?}", accounts);

    Ok(())
}
