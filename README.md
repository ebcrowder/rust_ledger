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

install via cargo - `cargo install rust_ledger`

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
  acct_offset_name: credit_card
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
  acct_offset_name: credit_card
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
Checking                     asset               
Savings                      asset               
CreditCard                   liability           
equity                       equity              
auto                         expense             
grocery                      expense             
fuel                         expense             
pets                         expense             
general                      expense             
general                      income              
income-1                     income              
interest                     income 
```

- balance
  - lists account balances to date
  - example output:

```
 Account                       Balance             
---------------------------------------
asset
  Checking                     2,400               
  Savings                      2,000               
liability
  CreditCard                   -456                
equity
  equity                       -3,500              
expense
  auto                         455                 
  grocery                      0                   
  fuel                         0                   
  pets                         0                   
  general                      1                   
income
  general                      -600                
  income-1                     -180                
  interest                     -120                

---------------------------------------
check                          0       
```

- register
  - lists general ledger transactions to date
  - can filter output by any field via optional parameter
  - example output:

```
Date       Description             Accounts            
-------------------------------------------------------------------------------
11/4/2019  car maintenance         auto                         455         455
                                   CreditCard                  -455           0
11/4/2019  raspberry pi            general                        1           1
                                   CreditCard                    -1           0
05/23/2020 business stuff          Checking                     600         600
                                   general                     -600           0
05/23/2020 business stuff          Checking                     300         300
                                   income-1                    -180         180
                                   interest                    -120           0
```

- csv
  - converts `csv` files to `yaml` format expected by `rust_ledger`
  - most financial institutions (banks, credit unions and credit card companies) will provide exports of transaction history in `csv` format
  - **note** - prior to importing your `csv` file into the tool, you must rename the columns in the first line of the `csv` file in the following schema:
    `"date","transaction","name","memo","amount"`

### rust_ledger `yaml` file format

- `rledger.yaml` example found at `examples/rledger.yaml`
- rust_ledger utilizes `yaml` files in the following format:

```yaml
owner: test-user
currencies:
  id: $
  name: US Dollar
  alias: USD
  note: Currency used in the United States

accounts:
  - id: 0
    acct_name: Checking
    acct_type: asset
    debit_credit: 1500
  - id: 1
    acct_name: Savings
    acct_type: asset
    debit_credit: 2000
  - id: 2
    acct_name: CreditCard
    acct_type: liability
    debit_credit: 0
  - id: 3
    acct_name: equity
    acct_type: equity
    debit_credit: -3500
  - id: 4
    acct_name: auto
    acct_type: expense
    debit_credit: 0
  - id: 5
    acct_name: grocery
    acct_type: expense
    debit_credit: 0
  - id: 6
    acct_name: fuel
    acct_type: expense
    debit_credit: 0
  - id: 7
    acct_name: pets
    acct_type: expense
    debit_credit: 0
  - id: 8
    acct_name: general
    acct_type: expense
    debit_credit: 0
  - id: 9
    acct_name: general
    acct_type: income
    debit_credit: 0
  - id: 10
    acct_name: income-1
    acct_type: income
    debit_credit: 0
  - id: 11
    acct_name: interest
    acct_type: income
    debit_credit: 0

transactions:
  - date: 11/4/2019
    debit_credit: 455
    acct_offset_name: CreditCard
    name: car maintenance
    acct_type: expense
    acct_name: auto
  - date: 11/4/2019
    debit_credit: 1
    acct_offset_name: CreditCard
    name: raspberry pi
    acct_type: expense
    acct_name: general
  - date: 05/23/2020
    debit_credit: 600
    acct_offset_name: Checking
    name: business stuff
    acct_type: income
    acct_name: general
  - date: 05/23/2020
    debit_credit: 300
    acct_offset_name: Checking
    name: business stuff
    acct_type: income
    acct_name:
    split:
      - amount: -180
        account: income-1
      - amount: -120
        account: interest
```
