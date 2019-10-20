// convert csv to yaml format

extern crate csv;

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct CSV {
    date: String,
    transaction: String,
    name: String,
    memo: String,
    amount: f64,
}

pub fn csv(filename: &str) -> Result<(), std::io::Error> {
    let file = fs::File::open(filename)?;
    let mut csv_reader = csv::Reader::from_reader(file);

    for result in csv_reader.deserialize() {
        let record: CSV = result?;
        println!("- date: {:?}", record.date);
        println!("transaction: {:?}", record.transaction);
        println!("debit_credit: {:?}", record.amount);

        // include acct_offset as credit_card acct
        println!("acct_offset_name: credit_card");

        // if negative, return expense acct - otherwise, return income acct
        if record.amount < 0.00 {
            println!("acct_name: expense-credit-card");
        } else {
            println!("acct_name: income-other");
        }
    }

    Ok(())
}
