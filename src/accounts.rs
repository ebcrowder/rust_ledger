// returns all general ledger accounts

use std::fs;

pub fn accounts(filename: &str) -> Result<(), std::io::Error> {
    let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");

    // split words by separate lines and collect them into a Vector
    let mut account_vec: Vec<&str> = file_string.split_ascii_whitespace().collect();
    // sort Vector and remove duplicates
    account_vec.sort();
    account_vec.dedup();

    for account in account_vec {
        if account.contains(':') {
            println!("{}", account);
        }
    }

    Ok(())
}
