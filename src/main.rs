mod cli;
mod error;
mod model;

#[macro_use]
extern crate prettytable;

fn main() -> Result<(), error::Error> {
    cli::run()
}
