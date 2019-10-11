// returns balances of all general ledger accounts

use std::collections::HashMap;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn balance(filename: &str) -> Result<(), std::io::Error> {
    fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("whoops");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("could not parse line"))
            .collect()
    };

    #[derive(Debug)]

    struct Accounts {
        account: String,
        amount: f32,
    }

    let mut transactions_vec: Vec<Accounts> = Vec::new();

    // iterate through text file and push transactions into vector
    let lines = lines_from_file(filename);
    let mut amount: f32 = 0.00;
    for line in lines {
        if line.contains(':') {
            let transaction: Vec<&str> = line.split_ascii_whitespace().collect();
            let account = transaction[0].to_string();

            if transaction.len() > 1 {
                amount = transaction[1].parse::<f32>().unwrap();
            }

            if transaction.len() == 1 {
                amount = -amount;
            }
            transactions_vec.push(Accounts { account, amount });
        }
    }

    println!("{:?}", transactions_vec);

    // summarize totals by account and place into HashMap
    let mut occurrences = HashMap::new();
    for transaction in transactions_vec {
        *occurrences.entry(transaction.account).or_insert(0.00) += transaction.amount;
    }

    // create output

    let mut assets_sum: f32 = 0.00;
    let mut liabilities_sum: f32 = 0.00;
    let mut equity_sum: f32 = 0.00;
    let mut income_sum: f32 = 0.00;
    let mut expenses_sum: f32 = 0.00;
    let mut check_figure: f32 = 0.00;

    for (key, val) in occurrences.iter() {
        if key.contains("Assets") {
            assets_sum += val;
            check_figure += val;
        }

        if key.contains("Liabilities") {
            liabilities_sum += val;
            check_figure += val;
        }

        if key.contains("Equity") {
            equity_sum += val;
            check_figure += val;
        }

        if key.contains("Expenses") {
            expenses_sum += val;
            check_figure += val;
        }

        if key.contains("Income") {
            income_sum += val;
            check_figure += val;
        }
    }

    println!("Assets: {:.2}", assets_sum);
    println!("Liabilities: {:.2}", liabilities_sum);
    println!("Equity: {:.2}", equity_sum);
    println!("Income: {:.2}", income_sum);
    println!("Expenses: {:.2}", expenses_sum);
    println!("===============");
    println!("Check: {:.2}", check_figure);

    Ok(())
}
