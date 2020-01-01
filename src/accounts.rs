// returns all general ledger accounts
extern crate serde_yaml;

use super::models::{LedgerFile};

struct BalanceAccount {
    account: String,
    account_type: String,
}

pub fn accounts(filename: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    let mut account_vec: Vec<BalanceAccount> = Vec::new();

    for account in deserialized_file.accounts {
        account_vec.push(BalanceAccount {
            account: account.acct_name,
            account_type: account.acct_type,
        });
    }

    println!("{0: <20} | {1: <20}", "account", "account_type");

    for account in account_vec {
        println!("{0: <20} | {1: <20}", account.account, account.account_type);
    }

    Ok(())
}
