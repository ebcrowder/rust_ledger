mod accounts;
mod balance;
mod error;
mod register;

use std::env;

fn main() -> Result<(), std::io::Error> {
    // collect args into a vector and assign them to vars
    let args: Vec<String> = env::args().collect();
    let (filename, command) = parse_args(&args);

    fn parse_args(args: &[String]) -> (&str, &str) {
        let filename = &args[1];
        let command = &args[2];

        (filename, command)
    }

    // match ledger commands
    match command {
        "accounts" => accounts::accounts(filename),
        "balance" => balance::balance(filename),
        "register" => register::register(filename),
        _ => error::error(),
    }
}
