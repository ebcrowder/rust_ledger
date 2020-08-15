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

`rust_ledger -f LEDGER_FILE_PATH COMMAND -o OPTION`

LEDGER_FILE_PATH (denoted by `-f`) - relative path to location of yaml ledger file

  - Optionally, the ledger file path can be set via the environment variable `RLEDGER_FILE` in lieu of specifying whenever the program is invoked.
  - If `-f` is provided with a file path the file provided will be used instead of any `RLEDGER_FILE` set.

```
RLEDGER_FILE=~/rledger.yaml rust_ledger balances
```

`RLEDGER_FILE` can be set as a system or user environment variable.

```
export RLEDGER_FILE="$HOME/rledger.yaml"
```

COMMAND - ledger command (account, balance, register, or csv)

OPTION (denoted by `-o`) - allows you to filter the output of the `register` command by account type. For example, if you wish to only see "expense" transactions in the output, you would pass in `expense` as the option here. Additionally, the csv tool uses this parameter for specifying the csv file to be parsed.

GROUP (denoted by `-g`) - allows you to group the output of the `register` command by `year` or `month`. 

OFFSET (denoted by `-s`) - required for csv tool. Specifies the offsetting account that each transaction should be posted against. 

### Environment Variables

`RLEDGER_FILE` - Path to rledger file. ex: `RLEDGER_FILE=~/rledger.yaml`

`NO_COLOR` - Disables color output. ex: `NO_COLOR=true`

### Features

#### Transactions

Transactions can be expressed in two different ways. One is a "simplified" format for transactions that only impact two accounts: 

```yaml
- date: 2020-01-01
  amount: 200
  offset_account: liability:credit_card_amex
  description: grocery store
  account: expense:expense_general
```

The sign (debit / credit) associated with the `offset_account` value is the opposite of the sign of the value contained in `amount` field.  

In the above example transaction, since `expense_general` was debited by 200, the `credit_card_amex` account will be credited by the same amount. 

Transactions that involve more than two accounts are expressed in the following manner:

```yaml
- date: 2020-01-01
  description: grocery store
  transactions:
    - amount: 20
      account: expense:general
    - amount: 180
      account: expense:grocery
    - amount: -200
      account: liability:credit_card_amex
```

Transactions that only involve two accounts can also be expressed in the above format. 

### Test

- `cargo test`

### API

- account
  - lists accounts
  - example output:

```
 Account                      
---------------------------------------
asset:cash_checking         
asset:cash_savings          
liability:credit_card_amex  
equity:equity               
expense:grocery             
expense:general             
expense:mortgage            
income:general
```

- balance
  - lists account balances to date
  - example output:

```
 Account                       Balance             
---------------------------------------
asset
  asset:cash_checking          $ -700.00           
  asset:cash_savings           $ 1000.00           
liability
  liability:credit_card_amex   $ -455.00           
equity
  equity:equity                $ -3500.00          
expense
  expense:grocery              $ 635.00            
  expense:general              $ 1020.00           
  expense:mortgage             $ 2000.00           
income
  income:general               0                   

---------------------------------------
check                          0           
```

- register
  - lists general ledger transactions to date
  - can filter output by any field via optional parameter
  - example output:

```
Date       Description             Accounts              
---------------------------------------------------------------------------------
2019-12-31 weekly groceries        grocery                  $ 455.00     $ 455.00
                                   credit_card_amex        $ -455.00            0
2020-01-01 mortage                 mortgage                $ 2000.00    $ 2000.00
                                   cash_checking          $ -2000.00            0
2020-01-01 stuff                   general                 $ 1000.00    $ 1000.00
                                   cash_savings           $ -1000.00            0
2020-01-01 grocery store           general                   $ 20.00      $ 20.00
                                   grocery                  $ 180.00     $ 200.00
                                   cash_checking           $ -200.00            0
```

- csv
  - converts `csv` files to `yaml` format expected by `rust_ledger`.
  - should be invoked with `-f`, `-o`, and `s` arguments. These include the rust_ledger file location (unless specified via environment variable),
  csv file location and account offset, respectively.
  - the account offset (`s` argument) would be the offset transaction that the csv transactions should be posted against.
  - the csv tool will look for existing transactions that have matching `description` fields and will populate the appropriate expense/income accounts
  for any matches. Non-matches will use a default of `expense:general` or `income:general`, which is determined based on the sign of the `amount` field
  contained in the transaction.
  - **note** - prior to importing your `csv` file into the tool, you must rename the columns in the first line of the `csv` file in the following schema:
    `"date","transaction","name","memo","amount"`.

### rust_ledger `yaml` file format

- example ledger `yaml` file can be found at `examples/example.yaml`
- rust_ledger utilizes `yaml` files in the following format:

```yaml
accounts:
  - account: 
    amount:  

transactions:
  - date: 
    amount: 
    description: 
    account: 
    offset_account: 
  - date: 
    description: 
    transactions: 
      - amount: 
        account: 
      - amount: 
        account: 
```

The ledger format schema is purposely lightweight. The only requirements are as follows:
- the `account` field should be expressed in the following format: `account_classification:account_name`.
- the `amount` field should be a number. It can include up to two (2) decimal points.  
- the `date` field should be in the following format: `YYYY-MM-DD`. 

