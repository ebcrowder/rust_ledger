mod accounts;
mod balance;
mod csv;
mod error;
mod models;
mod register;

use pargs;
use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let command_args: Vec<String> = vec![
        String::from("accounts"),
        String::from("balance"),
        String::from("register"),
        String::from("csv"),
    ];

    let flag_args: Vec<String> = vec![];
    let option_args: Vec<String> = vec![String::from("-l"), String::from("-f")];

    let pargs_result = pargs::parse(args, command_args, flag_args, option_args)?;

    let empty_vec = &vec!["".to_string()];

    let pargs_options = match pargs_result.get("option_args") {
        Some(option_args_vec) => option_args_vec,
        _ => empty_vec,
    };

    let ledger_file = &pargs_options.clone()[1];

    let pargs_commands = match pargs_result.get("command_args") {
        Some(command_args_vec) => command_args_vec,
        _ => empty_vec,
    };

    match &pargs_commands[0][..] {
        "accounts" => accounts::accounts(ledger_file),
        "balance" => balance::balance(ledger_file),
        "register" => register::register(ledger_file, &pargs_options),
        "csv" => csv::csv(ledger_file, &pargs_options),
        _ => error::error(),
    }
}
