extern crate clap;
use crate::model::ledger::Group;
use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

pub struct Args {
    pub ledger_file: String,
    pub options_arg: String,
    pub group_arg: Group,
    pub offset_arg: String,
    pub command: Command,
}

pub enum Command {
    Account,
    Balance,
    Register,
    CSV,
    None,
}

impl Args {
    pub fn new() -> Args {
        Args {
            ledger_file: String::from(""),
            options_arg: String::from(""),
            group_arg: Group::None,
            offset_arg: String::from(""),
            command: Command::None,
        }
    }

    fn resolve_ledger_file(&mut self, sub: &ArgMatches) {
        self.ledger_file = match sub.value_of("filename") {
            Some(f) => String::from(f),
            None => match std::env::var("RLEDGER_FILE") {
                Ok(p) => p,
                Err(err) => format!("{}", err),
            },
        };
    }

    pub fn populate_args(&mut self) {
        let matches = App::new("rust_ledger")
            .version(crate_version!())
            .author("Eric Crowder <ebcrowder@gmail.com>")
            .subcommand(
                SubCommand::with_name("account")
                    .about("account module")
                    .arg(
                        Arg::with_name("filename")
                            .short("f")
                            .long("filename")
                            .help("location of ledger file")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("balance")
                    .about("balance module")
                    .arg(
                        Arg::with_name("filename")
                            .short("f")
                            .long("filename")
                            .help("location of ledger file")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("register")
                    .about("register module")
                    .arg(
                        Arg::with_name("filename")
                            .short("f")
                            .long("filename")
                            .help("location of ledger file")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("option")
                            .short("o")
                            .long("option")
                            .help("filter output by optional value")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("group")
                            .short("g")
                            .long("group")
                            .help("group register output by value")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("csv")
                    .about("csv module")
                    .arg(
                        Arg::with_name("filename")
                            .short("f")
                            .long("filename")
                            .help("location of ledger file")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("csv")
                            .short("c")
                            .long("csv")
                            .help("path of csv file")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("offset")
                            .short("o")
                            .long("offset")
                            .help("offset account for each csv transaction")
                            .takes_value(true),
                    ),
            )
            .get_matches();

        if let Some(sub) = matches.subcommand_matches("register") {
            Args::resolve_ledger_file(self, sub);
            self.options_arg = match sub.value_of("option") {
                Some(v) => String::from(v),
                None => String::from(""),
            };
            self.group_arg = match sub.value_of("group") {
                Some(v) => match v {
                    "month" => Group::Month,
                    "year" => Group::Year,
                    _ => Group::None,
                },
                None => Group::None,
            };
        }

        if let Some(sub) = matches.subcommand_matches("csv") {
            Args::resolve_ledger_file(self, sub);
            self.options_arg = match sub.value_of("csv") {
                Some(v) => String::from(v),
                None => String::from(""),
            };
            self.offset_arg = match sub.value_of("offset") {
                Some(v) => String::from(v),
                None => String::from(""),
            };
        }

        if let Some(sub) = matches.subcommand_matches("account") {
            Args::resolve_ledger_file(self, sub);
        }

        if let Some(sub) = matches.subcommand_matches("balance") {
            Args::resolve_ledger_file(self, sub);
        }

        match matches.subcommand_name() {
            Some("account") => self.command = Command::Account,
            Some("balance") => self.command = Command::Balance,
            Some("register") => self.command = Command::Register,
            Some("csv") => self.command = Command::CSV,
            _ => self.command = Command::None,
        };
    }
}
