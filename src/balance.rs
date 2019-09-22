// returns balances of all general ledger accounts

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn balance(filename: &str) -> Result<(), Error> {
    #[allow(dead_code)]
    struct AccountSum {
        account: String,
        balance: i32,
    }

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut accounts_vec: Vec<AccountSum> = Vec::new();

    for line in reader.lines() {
        let line_unwrap = line.unwrap();

        let line_vec: Vec<_> = line_unwrap.split_terminator("$").collect();

        accounts_vec.push(AccountSum {
            account: line_vec[0].parse::<String>().unwrap(),
            balance: line_vec[1].parse::<i32>().unwrap(),
        });

        println!("{:?}", line_vec);


    }

    // for account in accounts_vec {
    //     println!("{:?}", account.account)
    // }

    Ok(())
}
