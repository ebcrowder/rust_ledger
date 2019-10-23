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
    date: String,
    debit_credit: f64,
    acct_name: String,
    acct_type: String,
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
    account: String,
    amount: f64,
}

struct TransactionAccounts {
    account: String,
    offset_account: String,
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
            account: account.acct_name,
            amount: account.debit_credit.round(),
        });
    }

    // push transactions into Vec
    for transaction in deserialized_file.transactions {
        transactions_vec.push(TransactionAccounts {
            account: transaction.acct_name,
            offset_account: transaction.acct_offset_name,
            amount: transaction.debit_credit.round(),
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
    for account in &accounts_vec {
        *occurrences.entry(account.account).or_insert(0.00) += account.amount;
    }

    for transaction in transactions_vec {
        *occurrences.entry(transaction.account).or_insert(0.00) += transaction.amount;
        if transaction.amount > 0.00 {
            *occurrences
                .entry(transaction.offset_account)
                .or_insert(0.00) += -transaction.amount
        } else {
            *occurrences
                .entry(transaction.offset_account)
                .or_insert(0.00) += transaction.amount
        }
    }

    // create output

    for (key, val) in occurrences.iter() {
        check_figure += val;
        println!("key {} val {}", key, val);
        // println!("{}", transactions_vec);

        for account in &accounts_vec {
            if key.to_string() == account.account {
                println!("match")
            }
        }
    }

    println!("check {}", check_figure);

    Ok(())
}
