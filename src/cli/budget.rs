extern crate serde_yaml;

use crate::error::Result;
use crate::model::ledger::{Group, LedgerFile};

/// generates budget to actual report for transactions
/// by month or year
pub fn budget(filename: &str, option: &str, group: Group) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_budget_actual(deserialized_file, option, group);

    Ok(())
}
