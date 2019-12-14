// returns all general ledger transactions

extern crate serde_yaml;

use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Currencies {
    id: String,
    name: String,
    alias: String,
    note: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Accounts {
    id: i32,
    acct_name: String,
    acct_type: String,
    debit_credit: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Transactions {
    date: String,
    debit_credit: i32,
    acct_name: String,
    acct_type: String,
    acct_offset_name: String,
    name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LedgerFile {
    owner: String,
    currencies: Currencies,
    accounts: Vec<Accounts>,
    transactions: Vec<Transactions>,
}

pub fn register(filename: &str, option: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    println!(
        "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <20}",
        "date", "debit", "acct_name", "acct_offset_name", "acct_memo"
    );

    let filtered_items: Vec<Transactions> = deserialized_file
        .transactions
        .into_iter()
        .filter(|x| {
            if option == "all" {
                true
            } else {
                x.acct_type == option
            }
        })
        .collect();

    for item in filtered_items {
        println!(
            "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <20}",
            item.date,
            item.debit_credit.to_formatted_string(&Locale::en),
            item.acct_name,
            item.acct_offset_name,
            item.name
        );
    }

    Ok(())
}
