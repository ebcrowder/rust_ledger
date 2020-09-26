mod account;
mod args;
mod balance;
mod budget;
mod csv;
mod register;

use crate::error::Result;
use args::{Args, Command};

pub fn run() -> Result<()> {
    let mut matches = Args::new();
    matches.populate_args();
    let Args {
        ledger_file,
        options_arg,
        offset_arg,
        group_arg,
        command,
    } = matches;

    match command {
        Command::Account => account::account(ledger_file.as_str()),
        Command::Balance => balance::balance(ledger_file.as_str()),
        Command::Budget => budget::budget(ledger_file.as_str(), options_arg.as_str(), group_arg),
        Command::Register => {
            register::register(ledger_file.as_str(), options_arg.as_str(), group_arg)
        }
        Command::CSV => csv::csv(
            ledger_file.as_str(),
            options_arg.as_str(),
            offset_arg.as_str(),
        ),
        Command::None => unreachable!(),
    }
}
