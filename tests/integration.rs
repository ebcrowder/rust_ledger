use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile;

use std::env;

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l")
        .arg("test/file/does/not/exist.txt")
        .arg("account");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn file_path_found_as_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();

    let account_yml = br#"
    accounts:
      - account: asset:cash_checking
        amount: 1500
      - account: asset:cash_savings
        amount: 2000
      - account: liability:credit_card_amex
        amount: 0
      - account: equity:equity
        amount: -3500
      - account: expense:grocery
        amount: 0
      - account: expense:general
        amount: 0

    transactions:
      - date: 11/4/2019
        amount: 455
        description: weekly groceries
        account: expense:grocery
        offset_account: liability:credit_card_amex
      - date: 06/21/2020
        description: grocery store
        transaction:
          - amount: 20
            account: expense:general
          - amount: 180
            account: expense:grocery
          - amount: -200
            account: asset:cash_checking
        "#;

    file.write_all(account_yml).unwrap();
    file.flush().unwrap();

    env::set_var("RLEDGER_FILE", file.path());

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("account");

    cmd.assert().success();

    Ok(())
}

#[test]
fn account_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();

    let account_yml = br#"
    accounts:
      - account: asset:cash_checking
        amount: 1500
      - account: asset:cash_savings
        amount: 2000
      - account: liability:credit_card_amex
        amount: 0
      - account: equity:equity
        amount: -3500
      - account: expense:grocery
        amount: 0
      - account: expense:general
        amount: 0

    transactions:
      - date: 11/4/2019
        amount: 455
        description: weekly groceries
        account: expense:grocery
        offset_account: liability:credit_card_amex
      - date: 06/21/2020
        description: grocery store
        transaction:
          - amount: 20
            account: expense:general
          - amount: 180
            account: expense:grocery
          - amount: -200
            account: asset:cash_checking
        "#;

    file.write_all(account_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l").arg(file.path()).arg("account");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("asset:cash_checking"));

    Ok(())
}

#[test]
fn balance_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();

    let account_yml = br#"
    accounts:
      - account: asset:cash_checking
        amount: 1500
      - account: asset:cash_savings
        amount: 2000
      - account: liability:credit_card_amex
        amount: 0
      - account: equity:equity
        amount: -3500
      - account: expense:grocery
        amount: 0
      - account: expense:general
        amount: 0

    transactions:
      - date: 11/4/2019
        amount: 455
        description: weekly groceries
        account: expense:grocery
        offset_account: liability:credit_card_amex
      - date: 06/21/2020
        description: grocery store
        transaction:
          - amount: 20
            account: expense:general
          - amount: 180
            account: expense:grocery
          - amount: -200
            account: asset:cash_checking
        "#;

    file.write_all(account_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l").arg(file.path()).arg("balance");
    cmd.assert().success().stdout(predicate::str::contains(
        "equity:equity                $ -3500.00 ",
    ));

    Ok(())
}

#[test]
fn register_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();

    let account_yml = br#"
    accounts:
      - account: asset:cash_checking
        amount: 1500
      - account: asset:cash_savings
        amount: 2000
      - account: liability:credit_card_amex
        amount: 0
      - account: equity:equity
        amount: -3500
      - account: expense:grocery
        amount: 0
      - account: expense:general
        amount: 0

    transactions:
      - date: 11/4/2019
        amount: 455
        description: weekly groceries
        account: expense:grocery
        offset_account: liability:credit_card_amex
      - date: 06/21/2020
        description: grocery store
        transaction:
          - amount: 20
            account: expense:general
          - amount: 180
            account: expense:grocery
          - amount: -200
            account: asset:cash_checking
        "#;

    file.write_all(account_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust_ledger");
    cmd.arg("-l").arg(file.path()).arg("register");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("06/21/2020"));

    Ok(())
}

#[test]
fn csv_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut yaml_file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();
    let mut csv_file = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();

    let raw_csv = br#"date,transaction,name,memo,amount
1/1/2000,DEBIT,Grocery store,12345,-55.0000"#;

    let balance_yml = br#"
    accounts:
      - account: liability:credit_card
        amount: 1500
      - account: equity:equity
        amount: -1500
      - account: expense:general
        amount: 0
    transactions:
      - account: expense:general
        amount: 55
        date: 2000-01-01
        offset_account: liability:credit_card
        description: Grocery store 
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
