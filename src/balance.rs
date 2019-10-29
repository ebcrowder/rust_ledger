// returns balances of all general ledger accounts

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

    // loop over Vecs and increment(+)/decrement(-) totals
    // for each transaction

    for transaction in &transactions_vec {
        for account in &mut accounts_vec {
            if account.account == transaction.account {
                account.amount += &transaction.amount;
            }
            if account.account == transaction.offset_account {
                account.amount -= &transaction.amount;
            }
        }
    }

    // create output

    let mut check_figure: f64 = 0.00;

    println!("{0: <20} | {1: <10}", "account", "balance");

    for account in accounts_vec {
        check_figure += account.amount;
        println!("{0: <20} | {1: <10}", account.account, account.amount);
    }

    println!("{0: <20} | {1: <10}", "check", check_figure);

    Ok(())
}
