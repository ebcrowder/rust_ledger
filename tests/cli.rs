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
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;

    writeln!(
        file,
        "2019/09/01 * Opening Balance
	Assets:Checking                   $1000
	Assets:Savings                         $500
	Liabilities:Mortgage                $-280000
	Liabilities:CarLoan                  $-15000
	Liabilities:Mastercard                      $-500
	Equity:Opening Balances

2019/09/03 * (DEBIT) STORMBREAKER BREWING PORTLAND OR
    ;85369439244194602677984; 05812; ; ; ;
    Expenses:Unknown                             -33
    Equity:Unknown

2019/09/03 * (DEBIT) SQU*SQ *THE GREAT NORT Portland OR
    ;55432869243200804052309; 05499; ; ; ;
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
Liabilities:Mastercard
Liabilities:Mortgage
",
    ));

    Ok(())
}
