mod test {
    use assert_cmd::Command;

    #[test]
    fn print_accounts_to_stdout() {
        let mut cmd = Command::cargo_bin("rust_ledger").unwrap();

        let assert = cmd
            .args(&["account", "-f", "./examples/example.yaml"])
            .assert();
        assert.success();
    }

    #[test]
    fn print_balances_to_stdout() {
        let mut cmd = Command::cargo_bin("rust_ledger").unwrap();

        let assert = cmd
            .args(&["balance", "-f", "./examples/example.yaml"])
            .assert();
        assert.success();
    }

    #[test]
    fn print_register_to_stdout() {
        let mut cmd = Command::cargo_bin("rust_ledger").unwrap();

        let assert = cmd
            .args(&["register", "-f", "./examples/example.yaml"])
            .assert();
        assert.success();
    }

    #[test]
    fn print_budget_to_stdout() {
        let mut cmd = Command::cargo_bin("rust_ledger").unwrap();

        let assert = cmd
            .args(&[
                "budget",
                "-f",
                "./examples/example.yaml",
                "-o",
                "2020",
                "-g",
                "yearly",
            ])
            .assert();
        assert.success();
    }
}
