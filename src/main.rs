mod cli;
mod error;
mod ledger;

#[macro_use]
extern crate prettytable;

fn main() -> Result<(), error::Error> {
    cli::run()
}
