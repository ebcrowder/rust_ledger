extern crate serde_yaml;

use colored::*;
use super::models::{LedgerFile, Transaction};

use monee::*;

/// returns all general ledger transactions
pub fn register(filename: &String, option: &String) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    println!(
        "\n{0: <10} {1: <24} {2: <20}",
        "Date".bold(),
        "Description".bold(),
        "Accounts".bold()
    );

    println!("{0:-<90}", "".bright_blue());

    let filtered_items: Vec<Transaction> = deserialized_file
        .transactions
        .into_iter()
        .filter(|x| match option.as_str() {
            "all" => true,
            _ => {
                x.acct_type.contains(option)
                    || x.date.contains(option)
                    || x.acct_name.contains(option)
                    || x.acct_offset_name.contains(option)
                    || x.name.contains(option)
                    || x.debit_credit.to_string().contains(option)
            }
        })
        .collect();

    for item in filtered_items {
        let mut credit: f64 = 0.0;

        match item.split {
            None => {
                match item.acct_type.as_ref() {
                    "income" => {
                        println!("{0: <10} {1: <24} {2: <20} {3: >16} {4: >16}",
                            item.date,
                            item.name.bold(),
                            item.acct_offset_name,
                            format!("{: >1}", money!(item.debit_credit, "USD")).bold(),
                            format!("{: >1}", money!(item.debit_credit, "USD")).bold()
                        );
                        println!(
                            "{0: <36}{1: <20} {2: >16} {3: >16}",
                            "",
                            item.acct_name,
                            format!("{: >1}", money!(-item.debit_credit, "USD")).bold(),
                            "0".bold() // hack for now. No need to do any math
                        );
                    },
                    _ => {
                        println!("{0: <10} {1: <24} {2: <20} {3: >16} {4: >16}",
                            item.date,
                            item.name.bold(),
                            item.acct_name,
                            format!("{: >1}", money!(item.debit_credit, "USD")).bold(),
                            format!("{: >1}", money!(item.debit_credit, "USD")).bold()
                        );
                        println!(
                            "{0: <36}{1: <20} {2: >16} {3: >16}",
                            "",
                            item.acct_offset_name,
                            format!("{: >1}", money!(-item.debit_credit, "USD")).bold(),
                            format!("{: >1}", money!(item.debit_credit - item.debit_credit, "USD")).bold()
                        );
                    },
                };
            },
            Some(split) => {
                match item.acct_type.as_ref() {
                    "income" => {
                        if let Some((last, elements)) = split.split_last() {        
                            println!("{0: <10} {1: <24} {2: <20} {3: >16} {4: >16}",
                                item.date,
                                item.name.bold(),
                                item.acct_offset_name,
                                format!("{: >1}", money!(item.debit_credit, "USD")).bold(),
                                format!("{: >1}", money!(item.debit_credit, "USD")).bold()
                            );
        
                            for i in elements {
                                credit -= i.amount;
                                println!(
                                    "{0: <36}{1: <20} {2: >16} {3: >16}",
                                    "",
                                    i.account,
                                    format!("{: >1}", money!(i.amount, "USD")).bold(),
                                    format!("{: >1}", money!(credit, "USD)")).bold()
                                );
                            }

                            credit -= last.amount;
                            let check: f64 = item.debit_credit - credit;
        
                            println!(
                                "{0: <36}{1: <20} {2: >16} {3: >16}",
                                "",
                                last.account,
                                format!("{: >1}", money!(last.amount, "USD")).bold(),
                                if check != 0.0 { 
                                    format!("{: >1}", money!(check, "USD")).red().bold()
                                } else { 
                                    check.to_string().bold()
                                }
                            );
                        }  
                    },
                    _ => {
                        if let Some((first, elements)) = split.split_first() {
                            credit += first.amount;
        
                            println!("{0: <10} {1: <24} {2: <20} {3: >16} {4: >16}",
                                item.date,
                                item.name.bold(),
                                first.account,
                                format!("{: >1}", money!(first.amount, "USD")).bold(),
                                format!("{: >1}", money!(first.amount, "USD")).bold()
                            );
        
                            for i in elements {
                                credit += i.amount;
                                println!(
                                    "{0: <36}{1: <20} {2: >16} {3: >16}",
                                    "",
                                    i.account,
                                    format!("{: >1}", money!(i.amount, "USD")).bold(),
                                    format!("{: >1}", money!(credit, "USD")).bold()
                                );
                            }
        
                            let check: f64 = item.debit_credit - credit;
        
                            println!(
                                "{0: <36}{1: <20} {2: >16} {3: >16}",
                                "",
                                item.acct_offset_name,
                                format!("{: >1}", money!(-item.debit_credit, "USD")).bold(),
                                if check != 0.0 {
                                    format!("{: >1}", money!(check, "USD")).red().bold()
                                } else { 
                                    (check).to_string().bold()
                                }
                            );
                        }  
                    }
                };                  
            }
        }
    }

    println!("\n");

    Ok(())
}
