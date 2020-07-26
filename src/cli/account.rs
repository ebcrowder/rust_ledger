extern crate serde_yaml;

use crate::error::Error;
use crate::model::ledger::LedgerFile;
use colored::*;

struct BalanceAccount {
    account: String,
}

/// returns all general ledger accounts
pub fn account(filename: &String) -> Result<(), Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    let mut account_vec: Vec<BalanceAccount> = Vec::new();

    for account in deserialized_file.accounts {
        account_vec.push(BalanceAccount {
            account: account.account,
        });
    }

    println!("\n {0: <29}", "Account");
    println!("{:-<39}", "".bright_blue());

    for account in account_vec {
        println!("{0: <28}", account.account);
    }

    println!("\n");

    Ok(())
}
