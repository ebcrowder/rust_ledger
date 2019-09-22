mod accounts;
mod balance;
mod error;

use std::env;

fn main() -> Result<(), std::io::Error> {
    // collect args into a Vector and assign them to vars
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
        // return a message to prompt user for command
        _ => error::error(),
    }
}
