extern crate clap;

use crate::error::Error;
use crate::ledger::Group;
use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};

pub struct Args {
    pub ledger_file: String,
    pub options_arg: String,
    pub group_arg: Group,
    pub offset_arg: String,
    pub invert_arg: bool,
    pub command: Command,
}

pub enum Command {
    Account,
    Balance,
    Budget,
    Register,
    Csv,
    None,
}

impl Args {
    pub fn new() -> Args {
        Args {
            ledger_file: String::from(""),
            options_arg: String::from(""),
            group_arg: Group::None,
            offset_arg: String::from(""),
            invert_arg: false,
            command: Command::None,
        }
    }

    fn resolve_ledger_file(&mut self, sub: &ArgMatches) {
        self.ledger_file = match sub.value_of("filename") {
            Some(f) => String::from(f),
            None => match std::env::var("RUST_LEDGER_FILE") {
                Ok(p) => p,
                Err(err) => Error::InvalidArg(err.to_string()).to_string(),
            },
        };
    }

    pub fn populate_args(&mut self) {
        let matches = App::new("rust_ledger")
            .version(crate_version!())
            .author("Eric Crowder <eric@ebcrowder.dev>")
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(
                SubCommand::with_name("account")
                    .about("account module")
                    .arg(
                        Arg::with_name("filename")
                            .short("f")
                            .long("filename")
                            .help("location of ledger file")
                            .takes_value(true)
                            .required(true),
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
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("budget")
                    .about("budget module")
                    .arg(
                        Arg::with_name("filename")
                            .short("f")
                            .long("filename")
                            .help("location of ledger file")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("option")
                            .short("o")
                            .long("option")
                            .help("filter output by optional value")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("group")
                            .short("g")
                            .long("group")
                            .help("group budget output by value")
                            .takes_value(true)
                            .possible_values(&["daily", "monthly", "yearly"])
                            .required(true),
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
                            .takes_value(true)
                            .required(true),
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
                            .possible_values(&["daily", "monthly", "yearly"])
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
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("csv")
                            .short("c")
                            .long("csv")
                            .help("path of csv file")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("offset")
                            .short("o")
                            .long("offset")
                            .help("offset account for each csv transaction")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("invert")
                            .short("i")
                            .long("invert")
                            .help("invert amount for each csv transaction")
                            .takes_value(false),
                    ),
            )
            .get_matches();

        if let Some(sub) = matches.subcommand_matches("register") {
            Args::resolve_ledger_file(self, sub);
            self.options_arg = sub.value_of("option").unwrap_or("").to_string();
            self.group_arg = match sub.value_of("group") {
                Some("yearly") => Group::Yearly,
                Some("monthly") => Group::Monthly,
                Some("daily") => Group::Daily,
                _ => Group::None,
            }
        }

        if let Some(sub) = matches.subcommand_matches("budget") {
            Args::resolve_ledger_file(self, sub);
            self.options_arg = sub.value_of("option").unwrap_or("").to_string();
            self.group_arg = match sub.value_of("group") {
                Some("yearly") => Group::Yearly,
                Some("monthly") => Group::Monthly,
                Some("daily") => Group::Daily,
                _ => Group::None,
            }
        }

        if let Some(sub) = matches.subcommand_matches("csv") {
            Args::resolve_ledger_file(self, sub);
            self.options_arg = sub.value_of("csv").unwrap_or("").to_string();
            self.offset_arg = sub.value_of("offset").unwrap_or("").to_string();
            self.invert_arg = sub.is_present("invert");
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
            Some("budget") => self.command = Command::Budget,
            Some("register") => self.command = Command::Register,
            Some("csv") => self.command = Command::Csv,
            _ => self.command = Command::None,
        };
    }
}
