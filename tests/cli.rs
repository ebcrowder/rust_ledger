use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;

#[test]
fn file_does_not_exist() -> Result<(), std::io::Error> {
    let mut cmd = Command::new("./target/debug/rust-ledger");
    cmd.arg("test/file/does/not/exist.txt").arg("accounts");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn accounts_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();

    let account_yml = br#"
        owner: test
        currencies:
            id: $
            name: US Dollar
            alias: USD
            note: US Currency 
        accounts:
            -   id: 0
                acct_name: operating
                acct_type: asset
                debit_credit: 1500
            -   id: 1
                acct_name: equity
                acct_type: equity
                debit_credit: -1500
        transactions:
            -   acct_name: expense-test-acct
                debit_credit: 1
                acct_type: expense 
                date: 2019-01-01
                acct_offset_name: credit_card
        "#;

    file.write_all(account_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust-ledger");
    cmd.arg(file.path()).arg("accounts");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("operating            | asset"));

    Ok(())
}

#[test]
fn balances_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();

    let balance_yml = br#"
        owner: test
        currencies:
            id: $
            name: US Dollar
            alias: USD
            note: US Currency 
        accounts:
            -   id: 0
                acct_name: operating
                acct_type: asset
                debit_credit: 1500
            -   id: 1
                acct_name: equity
                acct_type: equity
                debit_credit: -1500
        transactions:
            -   acct_name: expense-test-acct
                debit_credit: 1
                acct_type: expense 
                date: 2019-01-01
                acct_offset_name: credit_card
        "#;

    file.write_all(balance_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust-ledger");
    cmd.arg(file.path()).arg("balance");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("equity               | -1500"));

    Ok(())
}

#[test]
fn register_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();

    let balance_yml = br#"
        owner: test
        currencies:
            id: $
            name: US Dollar
            alias: USD
            note: US Currency 
        accounts:
            -   id: 0
                acct_name: operating
                acct_type: asset
                debit_credit: 1500
            -   id: 1
                acct_name: equity
                acct_type: equity
                debit_credit: -1500
        transactions:
            -   acct_name: expense-test-acct
                debit_credit: 1
                acct_type: expense 
                date: 2019-01-01
                acct_offset_name: credit_card
        "#;

    file.write_all(balance_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust-ledger");
    cmd.arg(file.path()).arg("register");
    cmd.assert().success().stdout(predicate::str::contains(
        "2019-01-01 | 1          | expense-test-acct    | credit_card",
    ));

    Ok(())
}
