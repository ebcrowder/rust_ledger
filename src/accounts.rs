// returns all general ledger accounts

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

struct BalanceAccounts {
    account: String,
}

pub fn accounts(filename: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    let mut account_vec: Vec<BalanceAccounts> = Vec::new();

    for account in deserialized_file.accounts {
        account_vec.push(BalanceAccounts {
            account: account.acct_name,
        });
    }

    for account in account_vec {
        println!("{:?}", account.account);
    }

    Ok(())
}
