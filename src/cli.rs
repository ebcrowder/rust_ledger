mod account;
mod balance;
mod csv;
mod register;

use crate::error::{Error, Result};
use crate::model::ledger::Group;
use pargs::*;
use std::env;

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // define expected args for pargs
    let expected_command_args: Vec<String> = vec![
        String::from("account"),
        String::from("balance"),
        String::from("register"),
        String::from("csv"),
    ];
    let expected_flag_args: Vec<String> = vec![];
    let expected_option_args: Vec<String> = vec![
        String::from("-f"),
        String::from("-o"),
        String::from("-g"),
        String::from("-s"),
    ];

    let Matches {
        command_args,
        option_args,
        ..
    } = pargs::parse(
        args,
        expected_command_args,
        expected_flag_args,
        expected_option_args,
    )?;

    let ledger_file_env = match std::env::var("RLEDGER_FILE") {
        Ok(p) => p,
        Err(err) => format!("{}", err),
    };

    let ledger_file = match option_args.get("-f") {
        Some(value) => value.as_str(),
        None => ledger_file_env.as_str(),
    };

    let options_arg = match option_args.get("-o") {
        Some(value) => String::from(value),
        None => String::from(""),
    };

    let group_arg = match option_args.get("-g") {
        Some(value) => match value.as_str() {
            "month" => Group::Month,
            "year" => Group::Year,
            _ => panic!("that group command was not recognized."),
        },
        None => Group::None,
    };

    let offset_arg = match option_args.get("-s") {
        Some(value) => String::from(value),
        None => String::from(""),
    };

    match &command_args.len() {
        0 => Err(Error::InvalidArg(String::from("please enter a command."))),
        _ => match &command_args[0][..] {
            "account" => account::account(ledger_file),
            "balance" => balance::balance(ledger_file),
            "register" => register::register(ledger_file, &options_arg, group_arg),
            "csv" => csv::csv(ledger_file, &options_arg, &offset_arg),
            _ => panic!("command not found."),
        },
    }
}
