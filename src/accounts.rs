extern crate serde_yaml;

use colored::*;
use super::models::LedgerFile;

struct BalanceAccount {
    account: String,
    account_type: String,
}

/// returns all general ledger accounts
pub fn accounts(filename: &String) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    let mut account_vec: Vec<BalanceAccount> = Vec::new();

    for account in deserialized_file.accounts {
        account_vec.push(BalanceAccount {
            account: account.acct_name,
            account_type: account.acct_type,
        });
    }

    println!("\n {0: <29} {1: <20}", "Account", "Type");
    println!("{:-<39}", "".bright_blue());

    for account in account_vec {
        println!(
            "{0: <28} {1: <20}",
            account.account,
            account.account_type);
    }

    println!("\n");

    Ok(())
}
