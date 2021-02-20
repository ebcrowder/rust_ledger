extern crate serde_yaml;

use crate::error::Result;
use crate::ledger::{Group, LedgerFile};

/// returns all general ledger transactions
pub fn register(filename: &str, option: &str, group: Group) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    if group == Group::None {
        LedgerFile::print_register(deserialized_file, option)
    } else {
        LedgerFile::print_register_group(deserialized_file, option, group)
    }

    Ok(())
}
