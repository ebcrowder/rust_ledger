// returns balances of all general ledger accounts

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn balance(filename: &str) -> Result<(), Error> {
    // edit this eventually
    #[allow(dead_code)]
    struct AccountSum {
        account: String,
        balance: i32,
    }

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut accountsVec: Vec<AccountSum>;

    for line in reader.lines() {
        for word in line.unwrap().split_terminator("$").collect() {
            println!("{:?}", word);

            accountsVec.push(AccountSum {
                account: word.swap_remove(0),
                balance: word.to_string(),
            });
        }
    }
    Ok(())
}
