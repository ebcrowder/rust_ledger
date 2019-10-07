// returns balances of all general ledger accounts

use std::collections::HashMap;
use std::fs;

pub fn balance(filename: &str) -> Result<(), std::io::Error> {
    let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");
    let account_vec: Vec<&str> = file_string.split('\n').collect();

    struct Accounts {
        account: String,
        amount: f32,
    }

    let mut transactions_vec: Vec<Accounts> = Vec::new();

    // iterate through text file and push transactions into vector
    for line in account_vec {
        if line.contains(':') {
            let transaction: Vec<&str> = line.split_ascii_whitespace().collect();

            if transaction.len() > 1 {
                let account = transaction[0].to_string();
                let amount = transaction[1].parse::<f32>().unwrap();

                transactions_vec.push(Accounts { account, amount });
            }
        }
    }

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
