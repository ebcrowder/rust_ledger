// convert csv to yaml format

extern crate csv;

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct CSV {
    date: String,
    transaction: String,
    name: String,
    memo: String,
    amount: f64,
}

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

pub fn csv(ledger_file: &str, csv_file: &str) -> Result<(), std::io::Error> {
    // TODO check for matches in ledger file and
    // map transactions from csv to existing expense accounts

    // open csv file
    let raw_csv_file = fs::File::open(csv_file)?;
    let mut csv_reader = csv::Reader::from_reader(raw_csv_file);

    // open ledger file
    let raw_ledger_file = std::fs::File::open(ledger_file)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(raw_ledger_file).unwrap();

    for result in csv_reader.deserialize() {
        let record: CSV = result?;
        if record.amount < 0.00 {
            // loop through transactions and find matching memos
            for transaction in &deserialized_file.transactions {
                if record.name == transaction.name {
                    println!("- date: {:?}", record.date);
                    println!("  debit_credit: {:?}", -record.amount.round() as i32);

                    // TODO make this not specific to my use case
                    println!("  acct_offset_name: credit_card");
                    println!("  name: {:?}", record.name);
                    println!("  acct_name: {}", transaction.acct_name);

                    // if negative, return expense acct
                    println!("  acct_type: expense");
                // println!("  acct_name: expense-credit-card");
                } else {
                    println!("- date: {:?}", record.date);
                    println!("  debit_credit: {:?}", -record.amount.round() as i32);

                    // TODO make this not specific to my use case
                    println!("  acct_offset_name: credit_card");
                    println!("  name: {:?}", record.name);

                    // if negative, return expense acct
                    println!("  acct_type: expense");
                    // println!("  acct_name: expense-credit-card");
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}
