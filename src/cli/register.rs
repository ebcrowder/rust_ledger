extern crate serde_yaml;

use crate::error::Result;
use crate::model::ledger::LedgerFile;

/// returns all general ledger transactions
pub fn register(filename: &String, option: &String) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_register(deserialized_file, option);

    Ok(())
}
