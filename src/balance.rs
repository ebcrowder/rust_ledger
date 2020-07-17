extern crate serde_yaml;

use super::models::LedgerFile;
use crate::error::LedgerError;
use colored::*;

struct BalanceAccount {
    account: String,
    amount: f64,
}

struct TransactionAccount {
    account: String,
    amount: f64,
}

/// returns balances of all general ledger accounts
pub fn balance(filename: &String) -> Result<(), LedgerError> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    let mut accounts_vec: Vec<BalanceAccount> = Vec::new();
    let mut transactions_vec: Vec<TransactionAccount> = Vec::new();

    // push opening balances into Vec
    for account in deserialized_file.accounts {
        accounts_vec.push(BalanceAccount {
            account: account.account,
            amount: account.amount,
        });
    }

    // push transactions into Vec
    for transaction in deserialized_file.transactions {
        let optional_account = match transaction.account {
            None => "".to_string(),
            Some(name) => name,
        };

        let optional_amount = match transaction.amount {
            None => 0.00,
            Some(number) => number,
        };

        let account_type: Vec<&str> = optional_account.split(":").collect();

        match transaction.transaction {
            None => {
                let offset_account = match transaction.offset_account {
                    None => "".to_string(),
                    Some(name) => name,
                };

                let amount = match account_type[0] {
                    "income" => -optional_amount,
                    _ => optional_amount,
                };

                transactions_vec.push(TransactionAccount {
                    account: optional_account,
                    amount,
                });

                if !offset_account.is_empty() {
                    transactions_vec.push(TransactionAccount {
                        account: offset_account,
                        amount: -amount,
                    });
                }
            }
            Some(split) => {
                let mut credit: f64 = 0.0;

                for i in split {
                    let amount = match account_type[0] {
                        "income" => -i.amount,
                        _ => i.amount,
                    };
                    credit += amount;
                    transactions_vec.push(TransactionAccount {
                        account: i.account,
                        amount: i.amount,
                    })
                }

                transactions_vec.push(TransactionAccount {
                    account: optional_account,
                    amount: optional_amount - credit,
                });
            }
        }
    }

    // loop over Vecs and increment(+)/decrement(-) totals
    // for each transaction

    for transaction in &transactions_vec {
        let transaction_account_type: Vec<&str> = transaction.account.split(":").collect();

        for account in &mut accounts_vec {
            let account_type: Vec<&str> = account.account.split(":").collect();

            if account.account.eq_ignore_ascii_case(&transaction.account)
                && account_type[0] == transaction_account_type[0]
            {
                account.amount += &transaction.amount;
            }
        }
    }

    // create output

    let mut check_figure: f64 = 0.0;

    println!("\n {0: <29} {1: <20}", "Account".bold(), "Balance".bold());

    println!("{0:-<39}", "".bright_blue());

    let mut current_account_type = String::new();

    for account in accounts_vec {
        check_figure += account.amount;
        let account_type: Vec<&str> = account.account.split(":").collect();

        if !current_account_type.eq(account_type[0]) {
            current_account_type = account_type[0].to_string();
            println!("{}", current_account_type);
        }

        println!(
            "  {0: <28} {1: <20}",
            account.account,
            if account.amount < 0.0 {
                format!("{0:.2}", account.amount).to_string().red().bold()
            } else if account.amount == 0.0 {
                account.amount.to_string().yellow().bold()
            } else {
                format!("{0:.2}", account.amount).to_string().bold()
            }
        );
    }

    println!("\n{:-<39}", "".bright_blue());
    print!("{: <30}", "check");
    if check_figure == 0.0 {
        print!(" {:<20}\n", check_figure.to_string().bold());
    } else {
        print!(" {:<20}\n", format!("{0:.2}", check_figure).red().bold());
    }

    println!("\n");

    Ok(())
}
