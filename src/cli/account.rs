extern crate serde_yaml;

use crate::error::Error;
use crate::model::ledger::LedgerFile;

/// returns all general ledger accounts
pub fn account(filename: &String) -> Result<(), Error> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_accounts(deserialized_file);

    Ok(())
}
