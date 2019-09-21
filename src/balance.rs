// returns balances of all general ledger accounts

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn balance(filename: &str) -> Result<(), Error> {
    // edit this eventually
    #[allow(dead_code)]
    struct AccountSum {
        account: String,
        balance: i32,
    }

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
   
    Ok(())
}
