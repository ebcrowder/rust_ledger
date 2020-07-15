extern crate serde_yaml;

use super::models::{LedgerFile, Transaction};
use colored::*;

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
                let optional_account = match &x.account {
                    None => "optional:account".to_string(),
                    Some(name) => name.to_string(),
                };

                let optional_offset_account = match &x.offset_account {
                    None => "optional:offset_account".to_string(),
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
            None => "optional:offset_account".to_string(),
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
                        println!(
                            "{0: <10} {1: <20}    {2: <20}    {3: >8}   {4: >8}",
                            item.date,
                            item.description.bold(),
                            offset_account_name,
                            format!("{0:.2}", optional_amount).to_string().bold(),
                            format!("{0:.2}", optional_amount).to_string().bold()
                        );
                        println!(
                            "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                            "",
                            account_name,
                            format!("-{0:.2}", optional_amount).to_string().bold(),
                            "0".bold() // hack for now. No need to do any math
                        );
                    }
                    _ => {
                        println!(
                            "{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                            item.date,
                            item.description.bold(),
                            account_name,
                            format!("{0:.2}", optional_amount).to_string().bold(),
                            format!("{0:.2}", optional_amount).to_string().bold()
                        );
                        println!(
                            "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                            "",
                            offset_account_name,
                            format!("-{0:.2}", optional_amount).to_string().bold(),
                            format!("{0:.2}", (optional_amount - optional_amount))
                                .to_string()
                                .bold()
                        );
                    }
                };
            }
            Some(split) => {
                match account_type {
                    "income" => {
                        if let Some((last, elements)) = split.split_last() {
                            println!(
                                "{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                                item.date,
                                item.description.bold(),
                                offset_account_name,
                                format!("{0:.2}", optional_amount).to_string().bold(),
                                format!("{0:.2}", optional_amount).to_string().bold()
                            );

                            for i in elements {
                                credit -= i.amount;
                                let i_account_vec: Vec<&str> = i.account.split(":").collect();
                                let i_account_name = i_account_vec[1];
                                println!(
                                    "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                    "",
                                    i_account_name,
                                    format!("{0:.2}", i.amount).to_string().bold(),
                                    format!("{0:.2}", credit).to_string().bold()
                                );
                            }

                            credit -= last.amount;
                            let check: f64 = optional_amount - credit;

                            let last_account_vec: Vec<&str> = last.account.split(":").collect();
                            let last_account_name = last_account_vec[1];

                            println!(
                                "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                "",
                                last_account_name,
                                format!("{0:.2}", last.amount).to_string().bold(),
                                if check != 0.0 {
                                    format!("{0:.2}", check).to_string().red().bold()
                                } else {
                                    check.to_string().bold()
                                }
                            );
                        }
                    }
                    _ => {
                        if let Some((first, elements)) = split.split_first() {
                            credit += first.amount;

                            let first_account_vec: Vec<&str> = first.account.split(":").collect();
                            let first_account_name = first_account_vec[1];

                            println!(
                                "{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}",
                                item.date,
                                item.description.bold(),
                                first_account_name,
                                format!("{0:.2}", first.amount).to_string().bold(),
                                format!("{0:.2}", first.amount).to_string().bold()
                            );

                            for i in elements {
                                credit += i.amount;
                                let i_account_vec: Vec<&str> = i.account.split(":").collect();
                                let i_account_name = i_account_vec[1];
                                println!(
                                    "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                    "",
                                    i_account_name,
                                    format!("{0:.2}", i.amount).to_string().bold(),
                                    format!("{0:.2}", credit).to_string().bold()
                                );
                            }

                            let check: f64 = optional_amount - credit;

                            println!(
                                "{0: <35}{1: <20}    {2: >8}    {3: >8}",
                                "",
                                offset_account_name,
                                format!("-{0:.2}", optional_amount).to_string().bold(),
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
