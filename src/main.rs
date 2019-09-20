mod accounts;

use std::env;

fn main() {
    // collect args into a Vector and assign them to vars
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let command;

    // check for command args - return if none provided
    if args.len() < 3 {
        return;
    } else {
        command = &args[2];
    }

    // match ledger commands
    match command.as_ref() {
        // read contents of CSV and print
        "accounts" => accounts::read_csv_to_string(filename),
        // return a message to prompt user for command
        _ => println!("please provide a command"),
    }
}
