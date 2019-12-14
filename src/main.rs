mod accounts;
mod balance;
mod csv;
mod error;
mod register;

extern crate getopts;

use getopts::Options;
use std::env;

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), std::io::Error> {
    // collect args into a vector and assign them to vars
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
    }
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
    };
    do_work(&input, output);
    // let ledger_file: &str;
    // let csv_file: &str;
    // let command: &str;

    // match args.len() {
    //     1 => error::error(),
    //     2 => error::error(),
    //     3 => {
    //         ledger_file = &args[1];
    //         command = &args[2];

    //         match command {
    //             "accounts" => accounts::accounts(ledger_file),
    //             "balance" => balance::balance(ledger_file),
    //             "register" => register::register(ledger_file),
    //             _ => error::error(),
    //         }
    //     }
    //     4 => {
    //         ledger_file = &args[1];
    //         csv_file = &args[2];
    //         command = &args[3];

    //         match command {
    //             "csv" => csv::csv(ledger_file, csv_file),
    //             _ => error::error(),
    //         }
    //     }
    //     _ => error::error(),
    // }

    Ok(())
}
