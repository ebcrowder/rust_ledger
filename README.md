![](https://github.com/ebcrowder/rust-ledger/workflows/rust_ledger/badge.svg)

# rust_ledger

command line accounting tool

As a former CPA, I could not resist building my own accounting system.

### Summary

- Spiritual port of ledger
- Smaller feature set based on typical use cases - primarily, _1)_ how much money have I spent during `xyz` time period and _2)_ what is my financial situation as of right now?
- Uses `yaml` files as data store
- Includes a tool to convert `csv` files to `yaml` format

### Use

- clone this repo
- if Rust is not installed on your machine, follow the instructions on how to do that here: https://www.rust-lang.org/tools/install
- run `cargo build --release` to compile the binary
- go to `/target/release` and copy the `rust_ledger` binary in your path - generally `/usr/bin` on most unix-based systems
- run `rust_ledger run LEDGER_FILE_PATH COMMAND OPTION` where the following:
  - LEDGER_FILE_PATH - relative path to location of yaml ledger file
  - COMMAND - ledger command (accounts, balance, register, or csv)
  - OPTION - allows you to filter the output of the `register` command by account type. For example, if you wish to only see "expense" transactions in the output, you would pass in `expense` as the option here.

### Test

- run `cargo test` to run the test suite

### API

- accounts
  - lists accounts
  - example output:

```
account              | account_type
checking             | asset
savings              | asset
credit_card          | liability
equity               | equity
expense_auto         | expense
expense_computer     | expense
expense_food         | expense
expense_gasoline     | expense
expense_pets         | expense
expense_amazon       | expense
expense_home         | expense
expense_general      | expense
income_general       | income
```

- balance
  - lists account balances to date
  - example output:

```
account_type         | account              | balance
asset                | checking             | 1,500
asset                | savings              | 2,000
liability            | credit_card          | -456
equity               | equity               | -3,500
expense              | expense_auto         | 455
expense              | expense_computer     | 1
expense              | expense_food         | 0
expense              | expense_gasoline     | 0
expense              | expense_pets         | 0
expense              | expense_amazon       | 0
expense              | expense_home         | 0
expense              | expense_general      | 0
income               | income_general       | 0
check                | 0         0
```

- register
  - lists general ledger transactions to date
  - can filter output by account via optional parameter
  - example output:

```
date       | debit      | acct_name            | acct_offset_name     | acct_memo
11/4/2019  | 455        | expense_auto         | credit_card          | car maintenance
11/4/2019  | 1          | expense_computer     | credit_card          | raspberry pi
```

- csv
  - converts `csv` files to `yaml` format expected by `rust_ledger`

### rust_ledger `yaml` file format

- rust_ledger utilizes `yaml` files in the following format:

```yaml
owner: test_owner
currencies:
  id: $
  name: US Dollar
  alias: USD
  note: Currency used in the United States

accounts:
  - id: 0
    acct_name: checking
    acct_type: asset
    debit_credit: 1500
  - id: 1
    acct_name: savings
    acct_type: asset
    debit_credit: 2000
  - id: 3
    acct_name: credit_card
    acct_type: liability
    debit_credit: 0
  - id: 4
    acct_name: equity
    acct_type: equity
    debit_credit: -3500
  - id: 5
    acct_name: expense_auto
    acct_type: expense
    debit_credit: 0
  - id: 6
    acct_name: expense_computer
    acct_type: expense
    debit_credit: 0
  - id: 7
    acct_name: expense_food
    acct_type: expense
    debit_credit: 0
  - id: 8
    acct_name: expense_gasoline
    acct_type: expense
    debit_credit: 0
  - id: 9
    acct_name: expense_pets
    acct_type: expense
    debit_credit: 0
  - id: 10
    acct_name: expense_amazon
    acct_type: expense
    debit_credit: 0
  - id: 11
    acct_name: expense_home
    acct_type: expense
    debit_credit: 0
  - id: 12
    acct_name: expense_general
    acct_type: expense
    debit_credit: 0
  - id: 13
    acct_name: income_general
    acct_type: income
    debit_credit: 0

transactions:
  - date: 11/4/2019
    debit_credit: 455
    acct_offset_name: credit_card
    name: car maintenance
    acct_type: expense
    acct_name: expense_auto
  - date: 11/4/2019
    debit_credit: 1
    acct_offset_name: credit_card
    name: raspberry pi
    acct_type: expense
    acct_name: expense_computer
```
