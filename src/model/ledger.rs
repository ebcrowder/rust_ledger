use colored::*;
use monee::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub account: String,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionList {
    pub account: String,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub date: String,
    pub account: Option<String>,
    pub amount: Option<f64>,
    pub description: String,
    pub offset_account: Option<String>,
    pub transaction: Option<Vec<TransactionList>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerFile {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

impl LedgerFile {
    pub fn print_accounts(self) {
        println!("{0: <29}", "Account");
        println!("{:-<39}", "".bright_blue());

        for account in self.accounts {
            println!("{0: <28}", account.account);
        }

        println!("\n");
    }

    pub fn print_balances(self) {
        let mut accounts_vec: Vec<Account> = Vec::new();
        let mut transactions_vec: Vec<Account> = Vec::new();

        // push opening balances into Vec
        for account in self.accounts {
            accounts_vec.push(Account {
                account: account.account,
                amount: account.amount,
            });
        }

        // push transactions into Vec
        for transaction in self.transactions {
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

                    transactions_vec.push(Account {
                        account: optional_account,
                        amount,
                    });

                    if !offset_account.is_empty() {
                        transactions_vec.push(Account {
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
                        transactions_vec.push(Account {
                            account: i.account,
                            amount: i.amount,
                        })
                    }

                    transactions_vec.push(Account {
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

        println!("{0: <29} {1: <20}", "Account".bold(), "Balance".bold());

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
                    format!("{: >1}", money!(account.amount, "USD"))
                        .to_string()
                        .red()
                        .bold()
                } else if account.amount == 0.0 {
                    account.amount.to_string().yellow().bold()
                } else {
                    format!("{: >1}", money!(account.amount, "USD"))
                        .to_string()
                        .bold()
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
    }

    pub fn print_register(self, option: &String) {
        println!(
            "\n{0: <10} {1: <23} {2: <22}",
            "Date".bold(),
            "Description".bold(),
            "Accounts".bold()
        );

        println!("{0:-<81}", "".bright_blue());

        let filtered_items: Vec<Transaction> = self
            .transactions
            .into_iter()
            .filter(|x| match option.as_str() {
                "all" => true,
                _ => {
                    let optional_account = match &x.account {
                        None => "optional:account".to_string(),
                        Some(name) => name.to_string(),
                    };

                    let optional_offset_account = match &x.offset_account {
                        None => "optional:account".to_string(),
                        Some(name) => name.to_string(),
                    };

                    let optional_amount = match x.amount {
                        None => 0.00,
                        Some(number) => number,
                    };

                    x.date.contains(option)
                        || optional_amount.to_string().contains(option)
                        || optional_account.contains(option)
                        || optional_offset_account.contains(option)
                        || x.description.contains(option)
                }
            })
            .collect();

        for item in filtered_items {
            let optional_account = match item.account {
                None => "optional:account".to_string(),
                Some(name) => name,
            };

            let optional_offset_account = match item.offset_account {
                None => "optional:account".to_string(),
                Some(name) => name,
            };

            let optional_amount = match item.amount {
                None => 0.00,
                Some(number) => number,
            };

            let mut credit: f64 = 0.0;

            let account_vec: Vec<&str> = optional_account.split(":").collect();
            let account_type = account_vec[0];
            let account_name = account_vec[1];

            let offset_account_vec: Vec<&str> = optional_offset_account.split(":").collect();
            let offset_account_name = offset_account_vec[1];

            match item.transaction {
                None => {
                    match account_type {
                        "income" => {
                            match &optional_offset_account[..] {
                                "optional:account" => continue,
                                _ => {
                                    println!(
                                        "{0: <10} {1: <23} {2: <20} {3: >12} {4: >12}",
                                        item.date,
                                        item.description.bold(),
                                        offset_account_name,
                                        format!("{: >1}", money!(optional_amount, "USD"))
                                            .to_string()
                                            .bold(),
                                        format!("{: >1}", money!(optional_amount, "USD"))
                                            .to_string()
                                            .bold()
                                    );
                                }
                            }
                            match &optional_account[..] {
                                "optional:account" => continue,
                                _ => {
                                    println!(
                                        "{0: <35}{1: <20} {2: >12} {3: >12}",
                                        "",
                                        account_name,
                                        format!("{: >1}", money!(-optional_amount, "USD"))
                                            .to_string()
                                            .bold(),
                                        "0".bold() // hack for now. No need to do any math
                                    );
                                }
                            }
                        }
                        _ => {
                            match &optional_account[..] {
                                "optional:account" => continue,
                                _ => {
                                    println!(
                                        "{0: <10} {1: <23} {2: <20} {3: >12} {4: >12}",
                                        item.date,
                                        item.description.bold(),
                                        account_name,
                                        format!("{: >1}", money!(optional_amount, "USD"))
                                            .to_string()
                                            .bold(),
                                        format!("{: >1}", money!(optional_amount, "USD"))
                                            .to_string()
                                            .bold()
                                    );
                                }
                            }
                            match &optional_offset_account[..] {
                                "optional:account" => continue,
                                _ => {
                                    println!(
                                        "{0: <35}{1: <20} {2: >12} {3: >12}",
                                        "",
                                        offset_account_name,
                                        format!("{: >1}", money!(-optional_amount, "USD"))
                                            .to_string()
                                            .bold(),
                                        format!(
                                            "{: >1}",
                                            money!((optional_amount - optional_amount), "USD")
                                        )
                                        .to_string()
                                        .bold()
                                    );
                                }
                            }
                        }
                    };
                }
                Some(split) => {
                    match account_type {
                        "income" => {
                            if let Some((last, elements)) = split.split_last() {
                                match &optional_offset_account[..] {
                                    "optional:account" => continue,
                                    _ => {
                                        println!(
                                            "{0: <10} {1: <23} {2: <20} {3: >12} {4: >12}",
                                            item.date,
                                            item.description.bold(),
                                            offset_account_name,
                                            format!("{: >1}", money!(optional_amount, "USD"))
                                                .to_string()
                                                .bold(),
                                            format!("{: >1}", money!(optional_amount, "USD"))
                                                .to_string()
                                                .bold()
                                        );
                                    }
                                }
                                for i in elements {
                                    credit -= i.amount;
                                    let i_account_vec: Vec<&str> = i.account.split(":").collect();
                                    let i_account_name = i_account_vec[1];

                                    match &i.account[..] {
                                        "optional:account" => continue,
                                        _ => {
                                            println!(
                                                "{0: <35}{1: <20} {2: >12} {3: >12}",
                                                "",
                                                i_account_name,
                                                format!("{: >1}", money!(i.amount, "USD"))
                                                    .to_string()
                                                    .bold(),
                                                format!("{: >1}", money!(credit, "USD"))
                                                    .to_string()
                                                    .bold()
                                            );
                                        }
                                    }
                                }

                                credit -= last.amount;
                                let check: f64 = optional_amount - credit;

                                let last_account_vec: Vec<&str> = last.account.split(":").collect();
                                let last_account_name = last_account_vec[1];

                                match &last.account[..] {
                                    "optional:account" => continue,
                                    _ => {
                                        println!(
                                            "{0: <35}{1: <20} {2: >12} {3: >12}",
                                            "",
                                            last_account_name,
                                            format!("{: >1}", money!(last.amount, "USD"))
                                                .to_string()
                                                .bold(),
                                            if check != 0.0 {
                                                format!("{: >1}", money!(check, "USD"))
                                                    .to_string()
                                                    .red()
                                                    .bold()
                                            } else {
                                                check.to_string().bold()
                                            }
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            if let Some((first, elements)) = split.split_first() {
                                credit += first.amount;

                                let first_account_vec: Vec<&str> =
                                    first.account.split(":").collect();
                                let first_account_name = first_account_vec[1];

                                match &first.account[..] {
                                    "optional:account" => continue,
                                    _ => {
                                        println!(
                                            "{0: <10} {1: <23} {2: <20} {3: >12} {4: >12}",
                                            item.date,
                                            item.description.bold(),
                                            first_account_name,
                                            format!("{: >1}", money!(first.amount, "USD"))
                                                .to_string()
                                                .bold(),
                                            format!("{: >1}", money!(first.amount, "USD"))
                                                .to_string()
                                                .bold()
                                        );
                                    }
                                }

                                for i in elements {
                                    credit += i.amount;
                                    let i_account_vec: Vec<&str> = i.account.split(":").collect();
                                    let i_account_name = i_account_vec[1];

                                    match &i.account[..] {
                                        "optional:account" => continue,
                                        _ => {
                                            println!(
                                                "{0: <35}{1: <20} {2: >12} {3: >12}",
                                                "",
                                                i_account_name,
                                                format!("{: >1}", money!(i.amount, "USD"))
                                                    .to_string()
                                                    .bold(),
                                                format!("{: >1}", money!(credit, "USD"))
                                                    .to_string()
                                                    .bold()
                                            );
                                        }
                                    }
                                }

                                let check: f64 = optional_amount - credit;

                                match &optional_offset_account[..] {
                                    "optional:account" => continue,
                                    _ => {
                                        println!(
                                            "{0: <35}{1: <20} {2: >12} {3: >12}",
                                            "",
                                            offset_account_name,
                                            format!("{: >1}", money!(-optional_amount, "USD"))
                                                .to_string()
                                                .bold(),
                                            if check != 0.0 {
                                                (check).to_string().red().bold()
                                            } else {
                                                (check).to_string().bold()
                                            }
                                        );
                                    }
                                }
                            }
                        }
                    };
                }
            }
        }

        println!("\n");
    }
}

#[cfg(test)]
#[test]
fn print_accounts_to_stdout() {
    let file: LedgerFile = LedgerFile {
        accounts: vec![
            Account {
                account: "assets:cash".to_string(),
                amount: 100.00,
            },
            Account {
                account: "expenses:foo".to_string(),
                amount: 0.00,
            },
        ],
        transactions: vec![Transaction {
            date: "2020-01-01".to_string(),
            account: Some("assets:cash".to_string()),
            amount: Some(10.00),
            description: "test".to_string(),
            offset_account: Some("expenses:foo".to_string()),
            transaction: None,
        }],
    };

    let result = LedgerFile::print_accounts(file);
    assert_eq!(result, ())
}

#[test]
fn print_balances_to_stdout() {
    let file: LedgerFile = LedgerFile {
        accounts: vec![
            Account {
                account: "assets:cash".to_string(),
                amount: 100.00,
            },
            Account {
                account: "expenses:foo".to_string(),
                amount: 0.00,
            },
        ],
        transactions: vec![Transaction {
            date: "2020-01-01".to_string(),
            account: Some("assets:cash".to_string()),
            amount: Some(10.00),
            description: "test".to_string(),
            offset_account: Some("expenses:foo".to_string()),
            transaction: None,
        }],
    };

    let result = LedgerFile::print_balances(file);
    assert_eq!(result, ())
}

#[test]
fn print_register_to_stdout() {
    let file: LedgerFile = LedgerFile {
        accounts: vec![
            Account {
                account: "assets:cash".to_string(),
                amount: 100.00,
            },
            Account {
                account: "expenses:foo".to_string(),
                amount: 0.00,
            },
        ],
        transactions: vec![Transaction {
            date: "2020-01-01".to_string(),
            account: Some("assets:cash".to_string()),
            amount: Some(10.00),
            description: "test".to_string(),
            offset_account: Some("expenses:foo".to_string()),
            transaction: None,
        }],
    };

    let result = LedgerFile::print_register(file, &"".to_string());
    assert_eq!(result, ())
}
