extern crate serde_yaml;

use super::models::{LedgerFile, Transaction};
use num_format::{Locale, ToFormattedString};

/// returns all general ledger transactions
pub fn register(filename: &String, options: &Vec<String>) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();
    let mut option = String::new();

    // should filter args - maybe via filter then step_by

    for arg in options {
        option = if !arg.contains("-") {
            arg.to_string()
        } else {
            "".to_string()
        };
    }

    println!(
        "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <20}",
        "date", "dr_cr", "acct_name", "acct_offset_name", "acct_memo"
    );

    let filtered_items: Vec<Transaction> = deserialized_file
        .transactions
        .into_iter()
        .filter(|x| match option.as_str() {
            "all" => true,
            _ => {
                x.acct_type.contains(&option)
                    || x.date.contains(&option)
                    || x.acct_name.contains(&option)
                    || x.acct_offset_name.contains(&option)
                    || x.name.contains(&option)
                    || x.debit_credit.to_string().contains(&option)
            }
        })
        .collect();

    for item in filtered_items {
        println!(
            "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <20}",
            item.date,
            item.debit_credit.to_formatted_string(&Locale::en),
            item.acct_name,
            item.acct_offset_name,
            item.name
        );
    }

    Ok(())
}
