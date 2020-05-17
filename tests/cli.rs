use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile;

use std::env;

#[test]
fn file_does_not_exist() -> Result<(), std::io::Error> {
    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l")
        .arg("test/file/does/not/exist.txt")
        .arg("accounts");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn file_path_found_as_env() -> Result<(), Box<dyn std::error::Error>> {
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
                name: 'expense transaction'
        "#;

    file.write_all(account_yml).unwrap();
    file.flush().unwrap();

    env::set_var("RLEDGER_FILE", file.path());

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("accounts");

    cmd.assert().success();

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
                name: 'expense transaction'
        "#;

    file.write_all(account_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l").arg(file.path()).arg("accounts");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("operating                    asset"));

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
                name: 'expense transaction'
        "#;

    file.write_all(balance_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l").arg(file.path()).arg("balances");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("  equity                       -1,500"));

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
                name: test memo
        "#;

    file.write_all(balance_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l")
        .arg(file.path())
        .arg("register")
        .arg("-f=credit_card");
    cmd.assert().success().stdout(predicate::str::contains(
        "2019-01-01 test memo               expense-test-acct              1           1",
    ));

    Ok(())
}

#[test]
fn csv_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut yaml_file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();
    let mut csv_file = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();

    let raw_csv = br#"date,transaction,name,memo,amount
1/1/2000,DEBIT,Grocery store,12345,-55.0000
1/1/2000,DEBIT,Computer store,12345,-100.9900
1/1/2000,DEBIT,Gasoline store,12345,-40.0000"#;

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
            -   acct_name: expense_general
                debit_credit: 55
                acct_type: expense 
                date: 2000-01-01
                acct_offset_name: credit_card
                name: Grocery store 
            -   acct_name: expense_general
                debit_credit: 101
                acct_type: expense 
                date: 2000-01-01
                acct_offset_name: credit_card
                name: Computer store
            -   acct_name: expense_general
                debit_credit: 40
                acct_type: expense 
                date: 2000-01-01
                acct_offset_name: credit_card
                name: Gasoline store
        "#;

    yaml_file.write_all(balance_yml).unwrap();
    yaml_file.flush().unwrap();

    csv_file.write_all(raw_csv).unwrap();
    csv_file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l")
        .arg(yaml_file.path())
        .arg("csv")
        .arg("-f")
        .arg(csv_file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "contents of csv file successfully applied to ledger yaml file",
    ));

    Ok(())
}
