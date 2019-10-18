// returns balances of all general ledger accounts

extern crate serde_yaml;

use std::collections::HashMap;

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

struct BalanceAccounts {
    account_type: String,
    amount: f64,
}

struct TransactionAccounts {
    account: String,
    amount: f64,
}

pub fn balance(filename: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    let mut accounts_vec: Vec<BalanceAccounts> = Vec::new();
    let mut transactions_vec: Vec<TransactionAccounts> = Vec::new();

    // push opening balances into Vec
    for account in deserialized_file.accounts {
        accounts_vec.push(BalanceAccounts {
            account_type: account.acct_type,
            amount: account.debit_credit,
        });
    }

    // push transactions into Vec
    for transaction in deserialized_file.transactions {
        transactions_vec.push(TransactionAccounts {
            account: transaction.acct_name,
            amount: transaction.debit_credit,
        })
    }

    // sum totals in accounts and transactions Vecs

    let mut assets_sum: f64 = 0.00;
    let mut liabilities_sum: f64 = 0.00;
    let mut equity_sum: f64 = 0.00;
    let mut income_sum: f64 = 0.00;
    let mut expenses_sum: f64 = 0.00;
    let mut check_figure: f64 = 0.00;

    // summarize totals and place into HashMap
    let mut occurrences = HashMap::new();
    for account in accounts_vec {
        *occurrences.entry(account.account_type).or_insert(0.00) += account.amount;
    }

    for transaction in transactions_vec {
        *occurrences.entry(transaction.account).or_insert(0.00) += transaction.amount;
    }

    // create output

    for (key, val) in occurrences.iter() {
        if key.contains("asset") {
            assets_sum += val;
            check_figure += val;
        }

        if key.contains("liability") {
            liabilities_sum += val;
            check_figure += val;
        }

        if key.contains("equity") {
            equity_sum += val;
            check_figure += val;
        }

        if key.contains("expense") {
            expenses_sum += val;
            check_figure += val;
        }

        if key.contains("income") {
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
