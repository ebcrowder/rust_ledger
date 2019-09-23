// returns balances of all general ledger accounts

// use regex::Regex;
use std::fs;

pub fn balance(filename: &str) -> Result<(), std::io::Error> {
    let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");
    let account_vec: Vec<&str> = file_string.split('\n').collect();
    #[derive(Debug)]

    struct Accounts {
        account: String,
        amount: String,
    }

    let mut transactions_vec: Vec<Accounts> = Vec::new();

    for line in account_vec {
        if line.contains(':') {
            let line_vec: Vec<&str> = line.split('\t').collect();
            let transaction: Vec<&str> = line_vec[0].trim().split_ascii_whitespace().collect();

            if transaction.len() > 1 {
                let account = transaction[0].to_string();
                let amount = transaction[1].to_string();

                println!("account {:?}", account);
                println!("amount {:?}", amount);

                transactions_vec.push(Accounts { account, amount })
            }
        }
    }

    println!("{:?}", transactions_vec);

    Ok(())
}
