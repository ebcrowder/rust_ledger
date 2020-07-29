use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub account: String,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionList {
    pub account: String,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub date: String,
    pub account: Option<String>,
    pub amount: Option<f64>,
    pub description: String,
    pub offset_account: Option<String>,
    pub transaction: Option<Vec<TransactionList>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerFile {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

impl LedgerFile {
    pub fn print_accounts(self) {
        println!("{0: <29}", "Account");
        println!("{:-<39}", "".bright_blue());

        for account in self.accounts {
            println!("{0: <28}", account.account);
        }

        println!("\n");
    }
}
