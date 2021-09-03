use chrono::NaiveDate;
use prettytable::{format, Table};
use rusty_money::{iso, Money};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

/// root data structure that contains the deserialized `LedgerFile` data
/// and associated structs
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LedgerFile {
    pub currency: String,
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
    Monthly,
    Yearly,
    Daily,
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
            None => "".to_string(),
            Some(name) => name.to_string(),
        };

        let offset_account = match &transaction.offset_account {
            None => "".to_string(),
            Some(name) => name.to_string(),
        };

        let amount = transaction.amount.unwrap_or(0.00);

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
/// via 2D HashMap. The HashMap is keyed by date and the related
/// values are a HashMap keyed by Account and the related
/// values are cumulative totals.
/// e.g. Key: "2020-01-01" -> <Key: "expense:general", Value: 100.00>
#[derive(Debug, PartialEq)]
struct GroupMap {
    group_map: HashMap<String, HashMap<String, f64>>,
}

impl GroupMap {
    fn new() -> GroupMap {
        GroupMap {
            group_map: HashMap::new(),
        }
    }

    fn populate_group_map(
        &mut self,
        date_string: String,
        account: String,
        amount: f64,
        transactions: Vec<TransactionList>,
    ) {
        if amount != 0.00 && !account.is_empty() {
            let account_ref = self.group_map.get_mut(&date_string);
            if let Some(r) = account_ref {
                *r.entry(account).or_insert(0.00) += amount;
            } else {
                let mut account_map = HashMap::new();
                account_map.insert(account, amount);
                self.group_map.insert(date_string.clone(), account_map);
            }
        } else {
            for t in transactions {
                let account_ref = self.group_map.get_mut(&date_string);
                if let Some(r) = account_ref {
                    *r.entry(t.account).or_insert(0.00) += amount;
                } else {
                    let mut account_map = HashMap::new();
                    account_map.insert(t.account, t.amount);
                    self.group_map.insert(date_string.clone(), account_map);
                }
            }
        }
    }
}

impl LedgerFile {
    /// obtain ISO 4217 currency for reference
    fn get_currency(&self) -> &'static rusty_money::iso::Currency {
        let c = &self.currency;

        match iso::find(c) {
            Some(n) => n,
            None => rusty_money::iso::USD,
        }
    }

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
                    } = OptionalKeys::match_optional_keys(x);

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
                    Group::Yearly => "%Y",
                    Group::Monthly => "%m",
                    Group::Daily => "%d",
                    Group::None => "%Y",
                };
                x.date.format(filter_period).to_string() == option
            })
            .filter(|x| {
                let OptionalKeys { account, .. } = OptionalKeys::match_optional_keys(x);
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
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Account", "Balance"]);

        let currency_code = self.get_currency();
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

        for account in accounts_vec {
            check_figure += account.amount;
            let account_type: Vec<&str> = account.account.split(':').collect();

            if !current_account_type.eq(account_type[0]) {
                current_account_type = account_type[0].to_string();
                table.add_row(row![current_account_type]);
            }

            table
                .add_row(row![r->account.account, Money::from_str(&account.amount.to_string(), currency_code).unwrap()]);
        }

        table.add_empty_row();
        table.add_row(row!["check", check_figure]);
        table.printstd();
    }

    pub fn print_register_group(self, option: &str, group: Group) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Date / Account", "Total"]);

        let currency_code = self.get_currency();
        let mut group_map = GroupMap::new();
        let filtered_transactions = LedgerFile::filter_transactions_by_option(self, option);

        for transaction in filtered_transactions {
            let OptionalKeys {
                amount,
                account,
                transactions,
                ..
            } = OptionalKeys::match_optional_keys(&transaction);

            let year = transaction.date.format("%Y").to_string();
            let month = transaction.date.format("%Y-%m").to_string();
            let day = transaction.date.format("%Y-%m-%d").to_string();

            match group {
                Group::Yearly => group_map.populate_group_map(year, account, amount, transactions),
                Group::Monthly => {
                    group_map.populate_group_map(month, account, amount, transactions)
                }
                Group::Daily => group_map.populate_group_map(day, account, amount, transactions),
                Group::None => (),
            }
        }

        for (date_string, account_map) in group_map.group_map.iter() {
            table.add_row(row![date_string]);
            for (account, amount) in account_map.iter() {
                table.add_row(row![
                    account,
                    Money::from_str(&amount.to_string(), currency_code).unwrap()
                ]);
            }
        }
        table.printstd();
    }

    pub fn print_register(self, option: &str) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Date", "Description", "Account", "Amount"]);

        let currency_code = self.get_currency();
        let filtered_transactions = LedgerFile::filter_transactions_by_option(self, option);

        for t in filtered_transactions {
            let OptionalKeys {
                account, amount, ..
            } = OptionalKeys::match_optional_keys(&t);

            table.add_row(row![
                t.date,
                t.description,
                account,
                Money::from_str(&amount.to_string(), currency_code).unwrap()
            ]);
        }
        table.printstd();
    }

    pub fn print_budget_actual(self, option: &str, group: Group) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Date / Account", "Budget", "Actual", "Delta"]);

        let currency_code = self.get_currency();
        let mut group_map = GroupMap::new();
        let filtered_transactions =
            LedgerFile::filter_income_expense_transactions(self.clone(), option, &group);

        for transaction in filtered_transactions {
            let OptionalKeys {
                amount,
                account,
                transactions,
                ..
            } = OptionalKeys::match_optional_keys(&transaction);

            let year = transaction.date.format("%Y").to_string();
            let month = transaction.date.format("%Y-%m").to_string();
            let day = transaction.date.format("%Y-%m-%d").to_string();

            match group {
                Group::Yearly => group_map.populate_group_map(year, account, amount, transactions),
                Group::Monthly => {
                    group_map.populate_group_map(month, account, amount, transactions)
                }
                Group::Daily => group_map.populate_group_map(day, account, amount, transactions),
                Group::None => (),
            }
        }

        for (date_string, account_map) in group_map.group_map.iter() {
            table.add_row(row![date_string]);
            for (account, amount) in account_map.iter() {
                let b = &Account {
                    account: "".to_string(),
                    amount: 0.0,
                    budget_month: None,
                    budget_year: None,
                };
                let matching_account = match self.accounts.iter().find(|x| &x.account == account) {
                    Some(a) => a,
                    None => b,
                };

                let budget = match &group {
                    Group::Yearly => matching_account.budget_year,
                    Group::Monthly => matching_account.budget_month,
                    Group::Daily => None, // not possible to set daily budgets
                    Group::None => None,
                };

                let budget_amount = budget.unwrap_or(0.00);
                let delta = budget_amount - amount;

                table.add_row(row![
                    account,
                    Money::from_str(&budget_amount.to_string(), currency_code).unwrap(),
                    Money::from_str(&amount.to_string(), currency_code).unwrap(),
                    Money::from_str(&delta.to_string(), currency_code).unwrap(),
                ]);
            }
        }
        table.printstd();
    }
}

#[cfg(test)]
fn get_file() -> LedgerFile {
    let date = match NaiveDate::parse_from_str("2020-01-01", "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e),
    };

    LedgerFile {
        currency: "USD".to_string(),
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
fn flatten_ledger_transactions() {
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
            },
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

        let day = transaction.date.format("%Y-%m-%d").to_string();

        group_map.populate_group_map(day, account, amount, transactions);
    }

    assert_eq!(
        group_map
            .group_map
            .get("2020-01-01")
            .unwrap()
            .get("expense:foo"),
        Some(&42.00)
    );
    assert_eq!(
        group_map
            .group_map
            .get("2020-01-01")
            .unwrap()
            .get("asset:cash"),
        Some(&-42.00)
    );
    assert_eq!(group_map.group_map.keys().count(), 1);
    assert_eq!(
        group_map
            .group_map
            .get("2020-01-01")
            .unwrap()
            .keys()
            .count(),
        2
    );
}
