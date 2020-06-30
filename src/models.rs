use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub account: String,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionList {
    pub amount: f64,
    pub account: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub date: String,
    pub amount: f64,
    pub account: String,
    pub description: String,
    pub offset_account: Option<String>,
    pub transaction: Option<Vec<TransactionList>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerFile {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}
