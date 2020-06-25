![](https://github.com/ebcrowder/rust-ledger/workflows/rust_ledger/badge.svg)
[![Latest version](https://img.shields.io/crates/v/rust_ledger.svg)](https://crates.io/crates/rust_ledger)
[![Documentation](https://docs.rs/rust_ledger/badge.svg)](https://docs.rs/rust_ledger)

# rust_ledger

command line accounting tool

As a former CPA, I could not resist building my own accounting system.

### Summary

- Spiritual port of [ledger](https://github.com/ledger/ledger)
- Uses double-entry accounting paradigm
- Small feature set based on typical use cases
- Uses `yaml` files as data store
- Includes a tool to convert `csv` files to `yaml` format

### Contributing
- See `CODE_OF_CONDUCT.md` for fundamental guidelines
- PRs, issues and feature requests are welcome and encouraged
- Join us on Matrix (#rust_ledger:matrix.org) at https://matrix.to/#/!dYISGJYNNiZcUrxhcm:matrix.org?via=matrix.org 

### Install

#### From Cargo 

`cargo install rust_ledger`

#### Build from Source

Alternatively, clone this repo and do the following:

- If Rust is not installed on your machine, follow the instructions on how to do that here: https://www.rust-lang.org/tools/install
- run `cargo build --release` to compile the binary
- go to `/target/release` and copy the `rust_ledger` binary in your path: `/usr/bin`

### Usage

`rust_ledger -l LEDGER_FILE_PATH COMMAND -f OPTION`

LEDGER_FILE_PATH (denoted by `-l`) - relative path to location of yaml ledger file

  - Optionally, the ledger file path can be set via the environment variable `RLEDGER_FILE` in lieu of specifying whenever the program is invoked.
  - If `-l` is provided with a file path the file provided will be used instead of any `RLEDGER_FILE` set.

```
RLEDGER_FILE=~/rledger.yaml rust_ledger balances
```

`RLEDGER_FILE` can be set as a system or user environment variable.

```
export RLEDGER_FILE="$HOME/rledger.yaml"
```

COMMAND - ledger command (accounts, balance, register, or csv)

OPTION (denoted by `-f`) - allows you to filter the output of the `register` command by account type. For example, if you wish to only see "expense" transactions in the output, you would pass in `expense` as the option here.

### Environment Variables

`RLEDGER_FILE` - Path to rledger file. ex: `RLEDGER_FILE=~/rledger.yaml`

`NO_COLOR` - Disables color output. ex: `NO_COLOR=true`

### Features

#### Transaction

```
- date: 05/23/2020
  debit_credit: 200
  acct_offset_name: credit_card_amex
  name: grocery store
  acct_type: expense
  acct_name: expense_general
```

**Required Fields**
* date
* debit_credit
* acct_offset_name
* name
* acct_type
* acct_name - This field is required but can be empty

#### Split Transactions

Each transaction can be split to multiple expense categories.

In order to add a split to a transaction add `split` to a transaction with `amount` and `account` added to each split.

Splits should add up to equal the `debit_credit`.

```
- date: 05/23/2020
  debit_credit: 200
  acct_offset_name: credit_card_amex
  name: grocery store
  acct_type: expense
  acct_name:
  split:
    - amount: 20
      account: expense_general
    - amount: 180
      account: expense_food
```

**Required Fields**
* amount
* account

### Test

- `cargo test`

### API

- accounts
  - lists accounts
  - example output:

```
 Account                       Type
---------------------------------------
cash_checking                asset
cash_savings                 asset
credit_card_amex             liability
equity                       equity
grocery                      expense
general                      expense
general                      income
```

- balance
  - lists account balances to date
  - example output:

```
 Account                       Balance             
---------------------------------------
asset
  cash_checking               $ 2100.00           
  cash_savings                $ 2000.00           
liability
  credit_card_amex            $ -705.00           
equity
  equity                      $ -3500.00          
expense
  grocery                     $ 635.00            
  general                     $ 70.00             
income
  general                     $ -600.00           

---------------------------------------
check                         0         
```

- register
  - lists general ledger transactions to date
  - can filter output by any field via optional parameter
  - example output:

```
Date       Description              Accounts            
------------------------------------------------------------------------------------------
11/4/2019  weekly groceries         grocery                      $ 455.00         $ 455.00
                                    credit_card_amex            $ -455.00                0
11/4/2019  raspberry pi             general                       $ 50.00          $ 50.00
                                    credit_card_amex             $ -50.00                0
05/23/2020 business stuff           cash_checking                $ 600.00         $ 600.00
                                    general                     $ -600.00                0
06/21/2020 grocery store            general                       $ 20.00          $ 20.00
                                    grocery                      $ 180.00         $ 200.00
                                    credit_card_amex            $ -200.00                0
```

- csv
  - converts `csv` files to `yaml` format expected by `rust_ledger`
  - most financial institutions (banks, credit unions and credit card companies) will provide exports of transaction history in `csv` format
  - **note** - prior to importing your `csv` file into the tool, you must rename the columns in the first line of the `csv` file in the following schema:
    `"date","transaction","name","memo","amount"`

### rust_ledger `yaml` file format

- example ledger `yaml` file can be found at `examples/example.yaml`
- rust_ledger utilizes `yaml` files in the following format:

```yaml
owner: user 
currencies:
  id: $
  name: US Dollar
  alias: USD
  note: Currency used in the United States

accounts:
  - id: 0
    acct_name: cash_checking
    acct_type: asset
    debit_credit: 1500
  - id: 1
    acct_name: cash_savings 
    acct_type: asset
    debit_credit: 2000
  - id: 2
    acct_name: credit_card_amex 
    acct_type: liability
    debit_credit: 0
  - id: 3
    acct_name: equity
    acct_type: equity
    debit_credit: -3500
  - id: 4
    acct_name: grocery
    acct_type: expense
    debit_credit: 0
  - id: 5
    acct_name: general
    acct_type: expense
    debit_credit: 0
  - id: 6
    acct_name: general
    acct_type: income
    debit_credit: 0

transactions:
  - date: 11/4/2019
    debit_credit: 455
    acct_offset_name: credit_card_amex
    name: weekly groceries
    acct_type: expense
    acct_name: grocery 
  - date: 11/4/2019
    debit_credit: 50
    acct_offset_name: credit_card_amex
    name: raspberry pi
    acct_type: expense
    acct_name: general
  - date: 05/23/2020
    debit_credit: 600
    acct_offset_name: cash_checking 
    name: business stuff
    acct_type: income
    acct_name: general
  - date: 06/21/2020
    debit_credit: 200
    acct_offset_name: credit_card_amex
    name: grocery store
    acct_type: expense
    acct_name:
    split:
      - amount: 20
        account: general
      - amount: 180
        account: grocery
```
