use chrono::NaiveDate;
use monee::*;
use prettytable::{format, Table};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

/// root data structure that contains the deserialized `LedgerFile` data
/// and associated structs
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LedgerFile {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account: String,
    pub amount: f64,
    pub budget_month: Option<f64>,
    pub budget_year: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub date: NaiveDate,
    pub account: Option<String>,
    pub amount: Option<f64>,
    pub description: String,
    pub offset_account: Option<String>,
    pub transactions: Option<Vec<TransactionList>>,
}

/// chrono::NaiveDate implements std::str::FromStr, so this is a generic
/// deserializer fn that can deserialize YAML strings into the NaiveDate struct
fn deserialize_date_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionList {
    pub account: String,
    pub amount: f64,
}

/// enumerates all possible `group` values for pattern matching
#[derive(Debug, PartialEq)]
pub enum Group {
    Month,
    Year,
    None,
}

/// data structure for handling `Option` values contained
/// within the `LedgerFile` for ease of program access
#[derive(Debug, PartialEq)]
struct OptionalKeys {
    account: String,
    offset_account: String,
    amount: f64,
    transactions: Vec<TransactionList>,
}

impl OptionalKeys {
    fn match_optional_keys(transaction: &Transaction) -> Self {
        let account = match &transaction.account {
            None => "optional:account".to_string(),
            Some(name) => name.to_string(),
        };

        let offset_account = match &transaction.offset_account {
            None => "optional:account".to_string(),
            Some(name) => name.to_string(),
        };

        let amount = match transaction.amount {
            None => 0.00,
            Some(number) => number,
        };

        let transactions = match transaction.transactions.clone() {
            None => vec![],
            Some(list) => list,
        };

        Self {
            account,
            offset_account,
            amount,
            transactions,
        }
    }
}

/// data structure for maintaining summarized register data
/// keyed by range
#[derive(Debug, PartialEq)]
struct GroupMap {
    group_map: HashMap<String, f64>,
}

impl GroupMap {
    fn new() -> GroupMap {
        GroupMap {
            group_map: HashMap::new(),
        }
    }

    fn populate_group_map(
        &mut self,
        range: String,
        amount: f64,
        transactions: Vec<TransactionList>,
    ) {
        let prev_value = match self.group_map.get(&range) {
            Some(value) => value,
            None => &0.00,
        };

        if amount != 0.00 {
            let inc_value = prev_value + amount;
            self.group_map.insert(range, inc_value);
        } else if !transactions.is_empty() {
            for t in transactions {
                self.group_map.insert(range.clone(), t.amount);
            }
        }
    }
}

impl LedgerFile {
    /// flatten abbreviated and detailed `LedgerFile` transactions into
    /// a Vec containing individual detailed transactions.
    /// all downstream logic expects this data structure.
    fn flatten_transactions(self) -> Vec<Transaction> {
        let mut flattened_transactions: Vec<Transaction> = Vec::new();

        for t in self.transactions {
            let OptionalKeys { amount, .. } = OptionalKeys::match_optional_keys(&t);
            match t.transactions {
                Some(subt) => {
                    for s in subt {
                        flattened_transactions.push(Transaction {
                            date: t.date,
                            account: Some(s.account),
                            amount: Some(s.amount),
                            transactions: None,
                            description: t.description.clone(),
                            offset_account: None,
                        });
                    }
                }
                None => {
                    // push entry
                    flattened_transactions.push(Transaction {
                        account: t.account.clone(),
                        offset_account: None,
                        amount: t.amount,
                        ..t.clone()
                    });

                    // push offset entry
                    flattened_transactions.push(Transaction {
                        account: t.offset_account,
                        offset_account: None,
                        amount: Some(amount * -1.00),
                        ..t
                    });
                }
            }
        }
        flattened_transactions
    }

    /// filter transactions by option. Downstream logic pairs this with
    /// the "group" argument for more extensive filtering
    fn filter_transactions_by_option(self, option: &str) -> Vec<Transaction> {
        let flattened_transactions = LedgerFile::flatten_transactions(self);

        flattened_transactions
            .into_iter()
            .filter(|x| match option {
                "" => true,
                _ => {
                    let OptionalKeys {
                        account,
                        offset_account,
                        amount,
                        ..
                    } = OptionalKeys::match_optional_keys(&x);

                    x.date.to_string().contains(option)
                        || amount.to_string().contains(option)
                        || account.contains(option)
                        || offset_account.contains(option)
                        || x.description.contains(option)
                }
            })
            .collect()
    }
    /// filters all income statement transactions by option
    fn filter_income_expense_transactions(self, option: &str, group: &Group) -> Vec<Transaction> {
        let flattened_transactions = LedgerFile::flatten_transactions(self);

        flattened_transactions
            .into_iter()
            .filter(|x| {
                let filter_period = match group {
                    Group::Month => "%m",
                    Group::Year => "%Y",
                    Group::None => "%Y",
                };
                x.date.format(filter_period).to_string() == option
            })
            .filter(|x| {
                let OptionalKeys { account, .. } = OptionalKeys::match_optional_keys(&x);
                account.contains("income") || account.contains("expense")
            })
            .collect()
    }

    pub fn print_accounts(self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Account"]);

        for account in self.accounts {
            table.add_row(row![account.account]);
        }
        table.printstd();
    }

    pub fn print_balances(self) {
        let mut accounts_vec: Vec<Account> = Vec::new();
        let mut transactions_vec: Vec<Account> = Vec::new();

        // push opening balances into Vec
        for account in &self.accounts {
            accounts_vec.push(Account {
                account: account.account.to_owned(),
                amount: account.amount.to_owned(),
                budget_month: None,
                budget_year: None,
            });
        }

        let flattened_transactions = LedgerFile::flatten_transactions(self);

        // push transactions into Vec
        for transaction in flattened_transactions {
            let OptionalKeys {
                account, amount, ..
            } = OptionalKeys::match_optional_keys(&transaction);

            transactions_vec.push(Account {
                account,
                amount,
                budget_month: None,
                budget_year: None,
            });
        }

        // loop over Vecs and increment(+)/decrement(-) totals
        // for each transaction
        for transaction in &transactions_vec {
            for account in &mut accounts_vec {
                if account.account.eq_ignore_ascii_case(&transaction.account) {
                    account.amount += &transaction.amount;
                }
            }
        }

        // create output
        let mut check_figure: f64 = 0.0;
        let mut current_account_type = String::new();

        println!("{0: <40} {1: >19}", "Account", "Balance");
        println!("{0:-<60}", "");

        for account in accounts_vec {
            check_figure += account.amount;
            let account_type: Vec<&str> = account.account.split(':').collect();

            if !current_account_type.eq(account_type[0]) {
                current_account_type = account_type[0].to_string();
                println!("{}", current_account_type);
            }

            println!(
                "  {0: <40} {1: >17}",
                account.account,
                if account.amount < 0.0 {
                    format!("{: >1}", money!(account.amount, "USD")).to_string()
                } else if account.amount == 0.0 {
                    account.amount.to_string()
                } else {
                    format!("{: >1}", money!(account.amount, "USD")).to_string()
                }
            );
        }

        println!("\n{:-<60}", "");
        print!("{: <58}", "check");

        if check_figure == 0.0 {
            println!(" {:<20}\n", check_figure.to_string());
        } else {
            println!(" {:<20}\n", format!("{0:.2}", check_figure));
        }

        println!("\n");
    }

    pub fn print_register_group(self, option: &str, group: Group) {
        let mut group_map = GroupMap::new();
        let filtered_transactions = LedgerFile::filter_transactions_by_option(self, option);

        println!("{0: <10} {1: <23} ", "Date", "Total");
        println!("{0:-<100}", "");

        for transaction in filtered_transactions {
            let OptionalKeys {
                amount,
                transactions,
                ..
            } = OptionalKeys::match_optional_keys(&transaction);

            let year = transaction.date.format("%Y").to_string();
            let month = transaction.date.format("%Y-%m").to_string();

            match group {
                Group::Month => group_map.populate_group_map(month, amount, transactions),
                Group::Year => group_map.populate_group_map(year, amount, transactions),
                Group::None => (),
            }
        }

        for (acct, amount) in group_map.group_map.iter() {
            println!(
                "{0: <10} {1: <23}",
                acct,
                format!("{: >1}", money!(amount, "USD")).to_string()
            );
        }
    }

    pub fn print_register(self, option: &str) {
        println!(
            "\n{0: <10} {1: <25} {2: <30} {3: >30}",
            "Date", "Description", "Account", "Amount"
        );

        println!("{0:-<100}", "");

        let filtered_transactions = LedgerFile::filter_transactions_by_option(self, option);

        for t in filtered_transactions {
            let OptionalKeys {
                account, amount, ..
            } = OptionalKeys::match_optional_keys(&t);

            println!(
                "{0: <10} {1: <25} {2: <30} {3: >30}",
                t.date,
                t.description,
                account,
                format!("{: >1}", money!(amount, "USD")).to_string(),
            );
        }

        println!("\n");
    }

    pub fn print_budget_actual(self, option: &str, group: Group) {
        let mut group_map = GroupMap::new();
        let filtered_transactions =
            LedgerFile::filter_income_expense_transactions(self.clone(), option, &group);

        println!(
            "{0: <20} {1: <35} {2: <20} {3: >10} ",
            "Date", "Budget", "Actual", "Delta"
        );
        println!("{0:-<100}", "");

        for transaction in filtered_transactions {
            let OptionalKeys {
                amount,
                account,
                transactions,
                ..
            } = OptionalKeys::match_optional_keys(&transaction);

            group_map.populate_group_map(account, amount, transactions)
        }

        for (acct, amount) in group_map.group_map.iter() {
            let b = &Account {
                account: "".to_string(),
                amount: 0.0,
                budget_month: None,
                budget_year: None,
            };
            let account = match self.accounts.iter().find(|x| &x.account == acct) {
                Some(a) => a,
                None => b,
            };

            let budget = match &group {
                Group::Month => account.budget_month,
                Group::Year => account.budget_year,
                Group::None => None,
            };

            let budget_amount = match budget {
                Some(a) => a,
                None => 0.00,
            };

            let delta = budget_amount - amount;

            println!(
                "{0: <20} {1: <35} {2: <20} {3: >10}",
                acct,
                format!("{: >1}", money!(budget_amount, "USD")).to_string(),
                format!("{: >1}", money!(amount, "USD")).to_string(),
                format!("{: >1}", money!(delta, "USD")).to_string()
            );
        }
    }
}

#[cfg(test)]
fn get_file() -> LedgerFile {
    let date = match NaiveDate::parse_from_str("2020-01-01", "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e),
    };

    LedgerFile {
        accounts: vec![
            Account {
                account: "asset:cash".to_string(),
                amount: 100.00,
                budget_month: None,
                budget_year: None,
            },
            Account {
                account: "expense:foo".to_string(),
                amount: 0.00,
                budget_month: None,
                budget_year: None,
            },
            Account {
                account: "expense:bar".to_string(),
                amount: 0.00,
                budget_month: None,
                budget_year: None,
            },
            Account {
                account: "expense:baz".to_string(),
                amount: 0.00,
                budget_month: None,
                budget_year: None,
            },
        ],
        transactions: vec![
            Transaction {
                date,
                account: Some("asset:cash".to_string()),
                amount: Some(10.00),
                description: "summary_transaction".to_string(),
                offset_account: Some("expense:foo".to_string()),
                transactions: None,
            },
            Transaction {
                date,
                account: Some("asset:cash".to_string()),
                amount: Some(-42.00),
                description: "summary_transaction".to_string(),
                offset_account: Some("expense:foo".to_string()),
                transactions: None,
            },
            Transaction {
                date,
                account: None,
                amount: None,
                description: "detailed_transaction".to_string(),
                offset_account: None,
                transactions: Some(vec![
                    TransactionList {
                        account: "asset:cash".to_string(),
                        amount: -50.00,
                    },
                    TransactionList {
                        account: "expense:bar".to_string(),
                        amount: 20.00,
                    },
                    TransactionList {
                        account: "expense:baz".to_string(),
                        amount: 30.00,
                    },
                ]),
            },
        ],
    }
}

#[test]
fn print_accounts_to_stdout() {
    let file = get_file();
    let result = LedgerFile::print_accounts(file);

    assert_eq!(result, ())
}

#[test]
fn print_balances_to_stdout() {
    let file = get_file();
    let result = LedgerFile::print_balances(file);

    assert_eq!(result, ())
}

#[test]
fn print_register_to_stdout() {
    let file = get_file();
    let result = LedgerFile::print_register(file, &"".to_string());

    assert_eq!(result, ())
}

#[test]
fn flatten_ledgerfile_transactions() {
    let file = get_file();
    let result = LedgerFile::flatten_transactions(file);

    assert_eq!(result.len(), 7)
}

#[test]
fn optional_keys() {
    let file = get_file();
    let result = OptionalKeys::match_optional_keys(&file.transactions[0].clone());

    assert_eq!(
        result,
        OptionalKeys {
            account: "asset:cash".to_string(),
            amount: 10.00,
            offset_account: "expense:foo".to_string(),
            transactions: vec![],
        }
    )
}

#[test]
fn filter_transactions_by_option_42() {
    let file = get_file();
    let result = LedgerFile::filter_transactions_by_option(file, &"42".to_string());
    let date = match NaiveDate::parse_from_str("2020-01-01", "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e),
    };

    assert_eq!(
        result,
        vec![
            Transaction {
                date,
                account: Some("asset:cash".to_string()),
                amount: Some(-42.00),
                description: "summary_transaction".to_string(),
                offset_account: None,
                transactions: None,
            },
            Transaction {
                date,
                account: Some("expense:foo".to_string()),
                amount: Some(42.00),
                description: "summary_transaction".to_string(),
                offset_account: None,
                transactions: None,
            }
        ]
    )
}

#[test]
fn group_map() {
    let file = get_file();
    let mut group_map = GroupMap::new();
    let filtered_transactions = LedgerFile::filter_transactions_by_option(file, &"42".to_string());
    for transaction in filtered_transactions {
        let OptionalKeys {
            amount,
            account,
            transactions,
            ..
        } = OptionalKeys::match_optional_keys(&transaction);

        group_map.populate_group_map(account, amount, transactions)
    }

    assert_eq!(group_map.group_map.get("expense:foo"), Some(&42.00));
    assert_eq!(group_map.group_map.get("asset:cash"), Some(&-42.00));
    assert_eq!(group_map.group_map.keys().len(), 2);
}
