extern crate csv;

use super::models::LedgerFile;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

#[derive(Debug, Deserialize)]
struct CSV {
    date: String,
    transaction: String,
    name: String,
    memo: String,
    amount: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CSVOutput {
    date: String,
    debit_credit: i32,
    acct_name: String,
    acct_type: String,
    acct_offset_name: String,
    name: String,
}

struct CSVMatches {
    acct_name: String,
    name: String,
}

fn write<T>(writer: &mut T, csv_output: &[CSVOutput]) -> Result<(), serde_yaml::Error>
where
    T: Write,
{
    serde_yaml::to_writer(writer, csv_output)?;
    Ok(())
}

fn write_ledger_file(ledger_file: &str, csv_output: &[CSVOutput]) -> Result<(), serde_yaml::Error> {
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(ledger_file)
        .unwrap();
    write(&mut f, csv_output)
}

fn insert_match_acct(csv_matches: &[CSVMatches], record: &CSV) -> String {
    for match_item in csv_matches {
        if match_item.name == record.name {
            return match_item.acct_name.to_string();
        }
    }
    if record.amount < 1.0 {
        "expense_general".to_string()
    } else {
        "income_general".to_string()
    }
}

/// convert csv to yaml format
pub fn csv(ledger_file: &str, csv_file: &str) -> Result<(), std::io::Error> {
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
                    name: transaction.name.to_string(),
                })
            }
        }
        // match memos with existing accounts in ledger yaml file
        let matched_acct_name = insert_match_acct(&csv_matches, &record);
        // push transaction to csv output Vector

        // if amount is negative, post as expense
        if record.amount < 1.0 {
            csv_output.push(CSVOutput {
                date: record.date,
                debit_credit: -record.amount.round() as i32,
                acct_name: matched_acct_name,
                acct_type: "expense".to_string(),
                acct_offset_name: "credit_card".to_string(),
                name: record.name,
            })
        } else {
            // if amount is positive, post as income
            csv_output.push(CSVOutput {
                date: record.date,
                debit_credit: record.amount.round() as i32,
                acct_name: matched_acct_name,
                acct_type: "income".to_string(),
                acct_offset_name: "credit_card".to_string(),
                name: record.name,
            })
        }
    }

    // write csv_output contents to ledger file
    write_ledger_file(ledger_file, &csv_output).unwrap();

    println!("contents of csv file successfully applied to ledger yaml file");

    Ok(())
}
