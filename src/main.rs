mod accounts;
mod balance;
mod csv;
mod error;
mod register;

use std::env;

fn main() -> Result<(), std::io::Error> {
    // collect args into a vector and assign them to vars
    let args: Vec<String> = env::args().collect();

    let ledger_file: &str;
    let csv_file: &str;
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

            if command == "csv" {
                csv_file = &args[3];
            } else {
                csv_file = "none";
            }

            if command == "register" {
                option = &args[3];
            } else {
                option = "none";
            }

            match command {
                "csv" => csv::csv(ledger_file, csv_file),
                "register" => register::register(ledger_file, option),
                _ => error::error(),
            }
        }
        _ => error::error(),
    }
}
