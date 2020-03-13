mod accounts;
mod balance;
mod csv;
mod error;
mod models;
mod register;

use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let ledger_file: &str;
    let command: &str;
    let option: &str;

    match args.len() {
        1 => error::error(),
        2 => error::error(),
        3 => {
            ledger_file = &args[1];
            command = &args[2];
            option = "all";

            match command {
                "accounts" => accounts::accounts(ledger_file),
                "balance" => balance::balance(ledger_file),
                "register" => register::register(ledger_file, option),
                _ => error::error(),
            }
        }
        4 => {
            ledger_file = &args[1];
            command = &args[2];

            match command {
                "csv" => csv::csv(ledger_file, &args[3]),
                "register" => register::register(ledger_file, &args[3]),
                _ => error::error(),
            }
        }
        _ => error::error(),
    }
}
