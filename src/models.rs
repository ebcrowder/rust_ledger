use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub note: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub acct_name: String,
    pub acct_type: String,
    pub debit_credit: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionSplit {
    pub amount: f64,
    pub account: String,
    pub account_type: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub date: String,
    pub debit_credit: f64,
    pub acct_name: String,
    pub acct_type: String,
    pub acct_offset_name: String,
    pub name: String,
    pub split: Option<Vec<TransactionSplit>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerFile {
    pub owner: String,
    pub currencies: Currency,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}
