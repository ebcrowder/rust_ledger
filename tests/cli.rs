use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

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
    let mut file = NamedTempFile::new()?;

    writeln!(
        file,
        "2019/09/01 * Opening Balance
	Assets:Checking                   $1000
	Assets:Savings                     $500
	Liabilities:Mortgage           $-100000
	Liabilities:CarLoan             $-15000
	Liabilities:Creditcard            $-500
	Equity:Opening Balances

2019/09/03 * (DEBIT) STORMBREAKER BREWING PORTLAND OR
    Expenses:Unknown                             -33
    Equity:Unknown

2019/09/03 * (DEBIT) SQU*SQ *THE GREAT NORT Portland OR
    Expenses:Unknown                           -2.25
    Equity:Unknown
"
    )?;

    let mut cmd = Command::new("./target/debug/rust-ledger");
    cmd.arg(file.path()).arg("accounts");
    cmd.assert().success().stdout(predicate::str::contains(
        "Assets:Checking
Assets:Savings
Equity:Opening
Equity:Unknown
Expenses:Unknown
Liabilities:CarLoan
Liabilities:Creditcard
Liabilities:Mortgage
",
    ));

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
                desc: test
                date: 2019-01-01
                acct_offset_name: credit_card
        "#;

    file.write_all(balance_yml).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::new("./target/debug/rust-ledger");
    cmd.arg(file.path()).arg("balance");
    cmd.assert().success().stdout(predicate::str::contains(
        "Assets: 1500.00
Liabilities: 0.00
Equity: -1500.00
Income: 0.00
Expenses: 1.00",
    ));

    Ok(())
}
