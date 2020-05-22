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
        println!(
            "{0: <10} {1: <20}    {2: <20}    {3: >8}    {4: >8}
                                   {5: <20}    {6: >8}    {7: >8}",
            item.date,
            item.name,
            item.acct_name,
            item.debit_credit.to_formatted_string(&Locale::en),
            format!("{}", item.debit_credit.to_formatted_string(&Locale::en)),
            item.acct_offset_name,
            format!("-{}", item.debit_credit.to_formatted_string(&Locale::en)),
            (item.debit_credit - item.debit_credit).to_formatted_string(&Locale::en)
        );
    }

    println!("\n");

    Ok(())
}
