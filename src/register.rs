extern crate serde_yaml;

use colored::*;
use super::models::{LedgerFile, Transaction};

/// returns all general ledger transactions
pub fn register(filename: &String, option: &String) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    println!(
        "\n{0: <10} {1: <23} {2: <20}",
        "Date".bold(),
        "Description".bold(),
        "Accounts".bold()
    );

    println!("{0:-<79}", "".bright_blue());

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
                        println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                            item.date,
                            item.name.bold(),
                            item.acct_offset_name,
                            format!("{0:.2}", item.debit_credit).to_string().bold(),
                            format!("{0:.2}", item.debit_credit).to_string().bold()
                        );
                        println!(
                            "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                            "",
                            item.acct_name,
                            format!("-{0:.2}", item.debit_credit).to_string().bold(),
                            "0".bold() // hack for now. No need to do any math
                        );
                    },
                    _ => {
                        println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                            item.date,
                            item.name.bold(),
                            item.acct_name,
                            format!("{0:.2}", item.debit_credit).to_string().bold(),
                            format!("{0:.2}", item.debit_credit).to_string().bold()
                        );
                        println!(
                            "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                            "",
                            item.acct_offset_name,
                            format!("-{0:.2}", item.debit_credit).to_string().bold(),
                            format!("{0:.2}", (item.debit_credit - item.debit_credit)).to_string().bold()
                        );
                    },
                };
            },
            Some(split) => {
                match item.acct_type.as_ref() {
                    "income" => {
                        if let Some((last, elements)) = split.split_last() {        
                            println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                                item.date,
                                item.name.bold(),
                                item.acct_offset_name,
                                format!("{0:.2}", item.debit_credit).to_string().bold(),
                                format!("{0:.2}", item.debit_credit).to_string().bold()
                            );
        
                            for i in elements {
                                credit -= i.amount;
                                println!(
                                    "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                    "",
                                    i.account,
                                    format!("{0:.2}", i.amount).to_string().bold(),
                                    format!("{0:.2}", credit).to_string().bold()
                                );
                            }

                            credit -= last.amount;
                            let check: f64 = item.debit_credit - credit;
        
                            println!(
                                "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                "",
                                last.account,
                                format!("{0:.2}", last.amount).to_string().bold(),
                                if check != 0.0 { 
                                    format!("{0:.2}", check).to_string().red().bold()
                                } else { 
                                    check.to_string().bold()
                                }
                            );
                        }  
                    },
                    _ => {
                        if let Some((first, elements)) = split.split_first() {
                            credit += first.amount;
        
                            println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                                item.date,
                                item.name.bold(),
                                first.account,
                                format!("{0:.2}", first.amount).to_string().bold(),
                                format!("{0:.2}", first.amount).to_string().bold()
                            );
        
                            for i in elements {
                                credit += i.amount;
                                println!(
                                    "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                    "",
                                    i.account,
                                    format!("{0:.2}", i.amount).to_string().bold(),
                                    format!("{0:.2}", credit).to_string().bold()
                                );
                            }
        
                            let check: f64 = item.debit_credit - credit;
        
                            println!(
                                "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                "",
                                item.acct_offset_name,
                                format!("-{0:.2}", item.debit_credit).to_string().bold(),
                                if check != 0.0 {
                                    (check).to_string().red().bold()
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
