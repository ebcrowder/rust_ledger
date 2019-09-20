mod accounts;
mod balance;
mod error;

use std::env;

fn main() -> Result<(), std::io::Error> {
    // collect args into a Vector and assign them to vars
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let command;

    // check for command args - return if none provided
    // if args.len() < 3 {
    //     return;
    // } else {
        command = &args[2];
    // }

    // match ledger commands
    match command.as_ref() {
        "accounts" => accounts::accounts(filename),
        "balance" => balance::balance(filename),
        // return a message to prompt user for command
        _ => error::error(),
    }
}
