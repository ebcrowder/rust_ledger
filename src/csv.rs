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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CSVOutput {
    date: String,
    debit_credit: f64,
    acct_name: String,
    acct_type: String,
    acct_offset_name: String,
    memo: String,
}

#[derive(Debug)]
struct CSVMatches {
    acct_name: String,
    memo: String,
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

    let mut csv_output: Vec<CSVOutput> = Vec::new();
    let mut csv_matches: Vec<CSVMatches> = Vec::new();

    for result in csv_reader.deserialize() {
        let record: CSV = result?;
        // loop through transactions and find matching memos

        for transaction in &deserialized_file.transactions {
            if transaction.name == record.name {
                csv_matches.push(CSVMatches {
                    acct_name: transaction.acct_name.to_string(),
                    memo: transaction.name.to_string(),
                })
            }
        }

        csv_output.push(CSVOutput {
            date: record.date,
            debit_credit: record.amount,
            acct_name: "test".to_string(),
            acct_type: "expense".to_string(),
            acct_offset_name: "liability-credit-card".to_string(),
            memo: record.name,
        })
    }

    for match_item in csv_matches {
        println!("{:?}", match_item);
    }

    let s = serde_yaml::to_string(&csv_output).unwrap();
    println!("{:?}", s);
    Ok(())
}
