extern crate csv;

use crate::error::Error;
use crate::ledger::LedgerFile;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{stdout, Write},
};

#[derive(Debug, Clone, Deserialize)]
struct Csv {
    date: String,
    description: String,
    name: String,
    amount: Option<f64>,
    debit: Option<f64>,
    credit: Option<f64>,
}

impl Csv {
    fn determine_amount(&self, invert: bool) -> f64 {
        if let Some(a) = self.amount {
            if invert {
                -a
            } else {
                a
            }
        } else if let Some(a) = self.debit {
            a
        } else if let Some(a) = self.credit {
            -a
        } else {
            // TODO parsing error should be emitted here
            panic!("csv file must have an account, debit or credit column")
        }
    }

    fn match_account(&self, amount: f64, ledger_file: &LedgerFile) -> String {
        for match_item in &ledger_file.transactions {
            let account = match &match_item.account {
                None => "".to_string(),
                Some(name) => name.to_string(),
            };

            if match_item.description == self.name {
                return account;
            }

            if amount.is_sign_positive() {
                return "expense:general".to_string();
            }
        }
        "income:general".to_string()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CsvOutput {
    records: Vec<CsvRecord>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CsvRecord {
    date: String,
    amount: f64,
    account: String,
    offset_account: String,
    description: String,
}

impl CsvOutput {
    fn new() -> CsvOutput {
        CsvOutput {
            records: Vec::new(),
        }
    }

    fn write<T>(&self, writer: &mut T) -> Result<(), serde_yaml::Error>
    where
        T: Write,
    {
        serde_yaml::to_writer(writer, self)?;
        Ok(())
    }

    fn write_to_stdout(self) -> Result<(), serde_yaml::Error> {
        CsvOutput::write(&self, &mut stdout())
    }

    fn populate_output_vec(
        &mut self,
        record: Csv,
        offset_account: &str,
        invert: bool,
        ledger_file: &LedgerFile,
    ) {
        let amount = Csv::determine_amount(&record, invert);
        let account = Csv::match_account(&record, amount, ledger_file);

        self.records.push(CsvRecord {
            date: record.date,
            account,
            amount,
            offset_account: offset_account.to_string(),
            description: record.name.trim().to_string(),
        })
    }
}

/// convert csv to yaml format
pub fn csv(
    ledger_file: &str,
    csv_file: &str,
    offset_account: &str,
    invert: bool,
) -> Result<(), Error> {
    // open csv file
    let raw_csv_file = fs::File::open(csv_file)?;
    let mut csv_reader = csv::Reader::from_reader(raw_csv_file);

    // open ledger file
    let raw_ledger_file = std::fs::File::open(ledger_file)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(raw_ledger_file).unwrap();

    let mut csv_output = CsvOutput::new();

    for result in csv_reader.deserialize() {
        let record: Csv = result?;

        csv_output.populate_output_vec(record, offset_account, invert, &deserialized_file);
    }

    // write csv_output contents to stdout
    CsvOutput::write_to_stdout(csv_output).unwrap();

    Ok(())
}

#[cfg(test)]
fn get_file() -> LedgerFile {
    use crate::ledger::{Account, Transaction, TransactionList};
    use chrono::NaiveDate;

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

/// negative `amount`s that do not have `name` matches should
/// be expense:general
#[test]
fn account_should_be_expense_general() {
    let file = get_file();
    let record = Csv {
        date: "2020-01-01".to_string(),
        description: "twin peaks diner coffee".to_string(),
        name: "coffee".to_string(),
        amount: Some(-2.50),
        debit: None,
        credit: None,
    };

    let amount = Csv::determine_amount(&record, true);
    let result = Csv::match_account(&record, amount, &file);

    assert_eq!(result, "expense:general");
}

/// positive `amount`s that do not have `name` matches should
/// be income:general
#[test]
fn account_should_be_income_general() {
    let file = get_file();
    let record = Csv {
        date: "2020-01-01".to_string(),
        description: "donuts sold to dale cooper".to_string(),
        name: "donuts".to_string(),
        amount: Some(2.50),
        debit: None,
        credit: None,
    };

    let amount = Csv::determine_amount(&record, true);
    let result = Csv::match_account(&record, amount, &file);

    assert_eq!(result, "income:general");
}

/// `name` matches should use the matched account `name`
#[test]
fn account_should_be_matched_account() {
    let file = get_file();
    let record = Csv {
        date: "2020-01-01".to_string(),
        description: "cherry pie sold to dale cooper".to_string(),
        name: "summary_transaction".to_string(),
        amount: Some(2.50),
        debit: None,
        credit: None,
    };

    let amount = Csv::determine_amount(&record, true);
    let result = Csv::match_account(&record, amount, &file);

    assert_eq!(result, "asset:cash");
}

/// negative `amount` should be expressed as debit
#[test]
fn negative_csv_amount_should_be_debit() {
    let file = get_file();
    let mut csv_output = CsvOutput::new();
    let account = "expense:general".to_string();
    let offset = "liability:amex".to_string();
    let record = Csv {
        date: "2020-01-01".to_string(),
        description: "twin peaks diner coffee".to_string(),
        name: "coffee".to_string(),
        amount: Some(-2.50),
        debit: None,
        credit: None,
    };

    csv_output.populate_output_vec(record.clone(), &offset, true, &file);

    let mut expected = CsvOutput {
        records: Vec::new(),
    };
    expected.records.push(CsvRecord {
        date: record.date,
        amount: -record.amount.unwrap(),
        account,
        offset_account: offset,
        description: record.name.trim().to_string(),
    });

    assert_eq!(csv_output, expected);
}

/// positive `amount` should be expressed as credit
#[test]
fn positive_csv_amount_should_be_credit() {
    let file = get_file();
    let mut csv_output = CsvOutput::new();
    let account = "income:general".to_string();
    let offset = "asset:cash".to_string();
    let record = Csv {
        date: "2020-01-01".to_string(),
        description: "coffee sold to dale cooper".to_string(),
        name: "coffee".to_string(),
        amount: Some(2.50),
        debit: None,
        credit: None,
    };

    csv_output.populate_output_vec(record.clone(), &offset, true, &file);

    let mut expected = CsvOutput {
        records: Vec::new(),
    };
    expected.records.push(CsvRecord {
        date: record.date,
        amount: -record.amount.unwrap(),
        account,
        offset_account: offset,
        description: record.name.trim().to_string(),
    });

    assert_eq!(csv_output, expected);
}

/// optional debits should be handled correctly
#[test]
fn should_handle_debits() {
    let file = get_file();
    let mut csv_output = CsvOutput::new();
    let account = "expense:general".to_string();
    let offset = "liability:amex".to_string();
    let record = Csv {
        date: "2020-01-01".to_string(),
        description: "coffee bought for dale cooper".to_string(),
        name: "coffee".to_string(),
        amount: None,
        debit: Some(2.50),
        credit: None,
    };

    csv_output.populate_output_vec(record.clone(), &offset, true, &file);

    let mut expected = CsvOutput {
        records: Vec::new(),
    };
    expected.records.push(CsvRecord {
        date: record.date,
        amount: record.debit.unwrap(),
        account,
        offset_account: offset,
        description: record.name.trim().to_string(),
    });

    assert_eq!(csv_output, expected);
}

/// optional credits should be handled correctly
#[test]
fn should_handle_credits() {
    let file = get_file();
    let mut csv_output = CsvOutput::new();
    let account = "income:general".to_string();
    let offset = "asset:cash".to_string();
    let record = Csv {
        date: "2020-01-01".to_string(),
        description: "coffee sold to dale cooper".to_string(),
        name: "coffee".to_string(),
        amount: None,
        debit: None,
        credit: Some(2.50),
    };

    csv_output.populate_output_vec(record.clone(), &offset, true, &file);

    let mut expected = CsvOutput {
        records: Vec::new(),
    };
    expected.records.push(CsvRecord {
        date: record.date,
        amount: -record.credit.unwrap(),
        account,
        offset_account: offset,
        description: record.name.trim().to_string(),
    });

    assert_eq!(csv_output, expected);
}
