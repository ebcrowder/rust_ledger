extern crate serde_yaml;

use crate::error::Error;
use crate::model::ledger::LedgerFile;

/// returns balances of all general ledger accounts
pub fn balance(filename: &String) -> Result<(), Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_balances(deserialized_file);

    Ok(())
}
