extern crate serde_yaml;

use rusty_money::{money, Money};

use colored::*;
use super::models::LedgerFile;
// use num_format::{Locale, ToFormattedString};

struct BalanceAccount {
    account: String,
    account_type: String,
    amount: f32,
}

struct TransactionAccount {
    account: String,
    offset_account: String,
    amount: f32,
}

/// returns balances of all general ledger accounts
pub fn balance(filename: &String) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    let mut accounts_vec: Vec<BalanceAccount> = Vec::new();
    let mut transactions_vec: Vec<TransactionAccount> = Vec::new();

    // push opening balances into Vec
    for account in deserialized_file.accounts {
        accounts_vec.push(BalanceAccount {
            account: account.acct_name,
            account_type: account.acct_type,
            amount: account.debit_credit,
        });
    }

    // push transactions into Vec
    for transaction in deserialized_file.transactions {
        let offset_account = &transaction.acct_offset_name;

        match transaction.split {
            None => {
                transactions_vec.push(TransactionAccount {
                    account: transaction.acct_name,
                    offset_account: offset_account.to_string(),
                    amount: transaction.debit_credit,
                });
            },
            Some(split) => {
                let mut credit: f32 = 0.0;
                
                for i in split {
                    credit += i.amount;
                    transactions_vec.push(TransactionAccount {
                        account: i.account,
                        offset_account: offset_account.to_string(),
                        amount: i.amount,
                    })
                }

                transactions_vec.push(TransactionAccount {
                    account: transaction.acct_name,
                    offset_account: offset_account.to_string(),
                    amount: transaction.debit_credit - credit,
                });
            }
        }
    }

    // loop over Vecs and increment(+)/decrement(-) totals
    // for each transaction

    for transaction in &transactions_vec {
        for account in &mut accounts_vec {
            if account.account.eq_ignore_ascii_case(&transaction.account) {
                account.amount += &transaction.amount;
            }
            if account.account.eq_ignore_ascii_case(&transaction.offset_account) {
                account.amount -= &transaction.amount;
            }
        }
    }

    // create output

    let mut check_figure: f32 = 0.0;

    println!("\n {0: <29} {1: <20}", "Account".bold(), "Balance".bold());

    println!("{0:-<39}", "".bright_blue());

    let mut current_account_type = String::new();

    for account in accounts_vec {
        check_figure += account.amount;

        if !current_account_type.eq(&account.account_type) {
            current_account_type = account.account_type;
            println!("{}", current_account_type);
        }

        println!(
            "  {0: <28} {1: <20}",
            account.account,
            if account.amount < 0.0 {
                money!(format!("{0:.2}", account.amount), "USD").to_string().red().bold()
            } else if account.amount == 0.0 {
                account.amount.to_string().yellow().bold()
            } else {
                money!(format!("{0:.2}", account.amount), "USD").to_string().bold()
            }
        );
    }

    println!("\n{:-<39}", "".bright_blue());
    print!("{: <30}", "check");
    if check_figure == 0.0 {
        print!(" {:<20}\n", format!("{0:.2}", check_figure).bold());
    } else {
        print!(" {:<20}\n", format!("{0:.2}", check_figure).red().bold());
    }

    println!("\n");

    Ok(())
}
