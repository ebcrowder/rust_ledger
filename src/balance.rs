// returns balances of all general ledger accounts
extern crate serde;
extern crate serde_yaml;

use num_format::{Locale, ToFormattedString};
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
    debit_credit: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Transactions {
    date: String,
    debit_credit: i32,
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
    account_type: String,
    amount: i32,
}

struct TransactionAccounts {
    account: String,
    offset_account: String,
    amount: i32,
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
            account_type: account.acct_type,
            amount: account.debit_credit,
        });
    }

    // push transactions into Vec
    for transaction in deserialized_file.transactions {
        transactions_vec.push(TransactionAccounts {
            account: transaction.acct_name,
            offset_account: transaction.acct_offset_name,
            amount: transaction.debit_credit,
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

    let mut check_figure: i32 = 0;

    println!(
        "{0: <20} | {1: <20} | {2: <10}",
        "account_type", "account", "balance"
    );

    for account in accounts_vec {
        check_figure += account.amount;
        println!(
            "{0: <20} | {1: <20} | {2: <10}",
            account.account_type,
            account.account,
            account.amount.to_formatted_string(&Locale::en)
        );
    }

    println!("{0: <20} | {1: <10}", "check", check_figure);

    Ok(())
}
