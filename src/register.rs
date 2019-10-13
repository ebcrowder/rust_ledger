// returns all general ledger transactions

extern crate serde_yaml;

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
    debit_credit: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Transactions {
    desc: String,
    date: String,
    debit_credit: f64,
    acct_name: String,
    acct_offset_name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LedgerFile {
    owner: String,
    currencies: Currencies,
    accounts: Vec<Accounts>,
    transactions: Vec<Transactions>,
}

pub fn register(filename: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    // TODO consider another way to do this
    println!("date              debit     acct_name                  acct_offset_name");

    for item in deserialized_file.transactions {
        println!(
            "{:?}      {:?}       {:?}        {:?}",
            item.date, item.debit_credit, item.acct_name, item.acct_offset_name
        );
    }

    Ok(())
}
