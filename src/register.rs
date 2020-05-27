extern crate serde_yaml;

use colored::*;
use rusty_money::{money, Money};
use super::models::{LedgerFile, Transaction, Currency};

/// returns all general ledger transactions
pub fn register(filename: &String, option: &String) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();
    let currencies: Currency = deserialized_file.currencies;

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
        let mut credit: f32 = 0.0;

        match item.split {
            None => {
                println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                    item.date,
                    item.name.bold(),
                    item.acct_name,
                    money!(format!("{0:.2}", item.debit_credit), currencies.alias).to_string().bold(),
                    money!(format!("{0:.2}", item.debit_credit), currencies.alias).to_string().bold()
                );

                println!(
                    "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                    "",
                    item.acct_offset_name,
                    money!(format!("-{0:.2}", item.debit_credit), currencies.alias).to_string().red().bold(),
                    0 // no check at this point because it can only be zero
                );
            },
            Some(split) => {
                if let Some((first, elements)) = split.split_first() {
                    credit += first.amount;

                    println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                        item.date,
                        item.name.bold(),
                        first.account,
                        money!(format!("{0:.2}", first.amount), currencies.alias).to_string().bold(),
                        money!(format!("{0:.2}", first.amount), currencies.alias).to_string().bold()
                    );

                    for i in elements {
                        credit += i.amount;
                        println!(
                            "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                            "",
                            i.account,
                            money!(format!("{0:.2}", i.amount), currencies.alias).to_string().bold(),
                            money!(format!("{0:.2}", credit), currencies.alias).to_string().bold()
                        );
                    }

                    let check: f32 = item.debit_credit - credit;

                    println!(
                        "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                        "",
                        item.acct_offset_name,
                        money!(format!("-{0:.2}", item.debit_credit), currencies.alias).to_string().red().bold(),
                        if check != 0.0 { 
                            money!(format!("{0:.2}", check), currencies.alias).to_string().red().bold() 
                        } else { 
                            check.to_string().bold()
                        }
                    );
                }                
            }
        }
    }

    println!("\n");

    Ok(())
}
