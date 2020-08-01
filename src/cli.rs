mod account;
mod balance;
mod csv;
mod register;

use crate::error::{Error, Result};
use crate::model::ledger::Group;
use pargs;
use std::env;

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // define expected args for pargs
    let command_args: Vec<String> = vec![
        String::from("account"),
        String::from("balance"),
        String::from("register"),
        String::from("csv"),
    ];
    let flag_args: Vec<String> = vec![];
    let option_args: Vec<String> = vec![String::from("-f"), String::from("-o"), String::from("-g")];

    // pargs will parse the args and return the result
    let pargs_result = pargs::parse(args, command_args, flag_args, option_args)?;

    let pargs_options = pargs_result.option_args;
    let pargs_commands = pargs_result.command_args;

    let ledger_file = match pargs_options.get("-f") {
        Some(value) => value.to_string(),
        None => {
            let ledger_file_env = match std::env::var("RLEDGER_FILE") {
                Ok(p) => format!("{}", p),
                Err(err) => format!("{}", err),
            };

            ledger_file_env.to_string()
        }
    };

    let options_arg = match pargs_options.get("-o") {
        Some(value) => value,
        None => "",
    };

    let group_arg = match pargs_options.get("-g") {
        Some(value) => match value.as_str() {
            "month" => Group::Month,
            "year" => Group::Year,
            _ => panic!("that group command was not recognized."),
        },
        None => Group::None,
    };

    match &pargs_commands.len() {
        0 => Err(Error::InvalidArg("please enter a command.".to_string())),
        _ => match &pargs_commands[0][..] {
            "account" => account::account(&ledger_file.to_string()),
            "balance" => balance::balance(&ledger_file.to_string()),
            "register" => register::register(
                &ledger_file.to_string(),
                &options_arg.to_string(),
                group_arg,
            ),
            "csv" => csv::csv(&ledger_file.to_string(), &options_arg.to_string()),
            _ => panic!("command not found.".to_string()),
        },
    }
}
