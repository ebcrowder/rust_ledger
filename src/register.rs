extern crate serde_yaml;

use colored::*;
use super::models::{LedgerFile, Transaction};
use num_format::{Locale, ToFormattedString};

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
        let mut credit: i32 = 0;

        match item.split {
            None => {
                println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                    item.date,
                    item.name.bold(),
                    item.acct_name,
                    item.debit_credit.to_formatted_string(&Locale::en).bold(),
                    format!("{}", item.debit_credit.to_formatted_string(&Locale::en)).bold()
                );
                println!(
                    "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                    "",
                    item.acct_offset_name,
                    format!("-{}", item.debit_credit.to_formatted_string(&Locale::en)).red().bold(),
                    (item.debit_credit - item.debit_credit).to_formatted_string(&Locale::en).bold()
                );
            },
            Some(split) => {
                if let Some((first, elements)) = split.split_first() {
                    credit += first.amount;

                    println!("{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                        item.date,
                        item.name.bold(),
                        first.account,
                        first.amount.to_formatted_string(&Locale::en).bold(),
                        format!("{}", first.amount.to_formatted_string(&Locale::en)).bold()
                    );

                    for i in elements {
                        credit += i.amount;
                        println!(
                            "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                            "",
                            i.account,
                            format!("{}", i.amount.to_formatted_string(&Locale::en)).bold(),
                            (credit).to_formatted_string(&Locale::en).bold()
                        );
                    }

                    let check = item.debit_credit - credit;

                    println!(
                        "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                        "",
                        item.acct_offset_name,
                        format!("-{}", item.debit_credit.to_formatted_string(&Locale::en)).red().bold(),
                        if check != 0 { (check).to_formatted_string(&Locale::en).red().bold() } else { (check).to_formatted_string(&Locale::en).bold() }
                    );
                }                
            }
        }
    }

    println!("\n");

    Ok(())
}
