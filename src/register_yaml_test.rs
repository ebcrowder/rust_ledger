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
struct LedgerFile {
    owner: String,
    currencies: Currencies,
    accounts: Vec<Accounts>,
    transactions: Vec<Transactions>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Accounts {
    id: i32,
    name: String,
    acct_type: String,
    debit_credit: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Transactions {
    acct_type: String,
    desc: String,
    date: String,
    debit_credit: f64,
}

pub fn register_yaml_test(filename: &str) -> Result<(), std::io::Error> {
    let f = std::fs::File::open(filename)?;
    let d: LedgerFile = serde_yaml::from_reader(f).unwrap();

    println!("Read YAML string: {:?}", d);

    Ok(())
}
