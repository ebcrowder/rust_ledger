![](https://github.com/ebcrowder/rust-ledger/workflows/rust_ledger/badge.svg)
[![Latest version](https://img.shields.io/crates/v/rust_ledger.svg)](https://crates.io/crates/rust_ledger)

# rust_ledger

command line accounting tool for Linux and macOS

### Summary

- Spiritual port of [ledger](https://github.com/ledger/ledger)
- Uses double-entry accounting paradigm
- Supports all [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency formats
- Uses `yaml` files as data store
- Includes a tool to convert `csv` files to `yaml` format
- Small feature set based on typical personal finance use cases

### Contributing

- See `CODE_OF_CONDUCT.md` for guidelines
- PRs, issues and feature requests are welcome and encouraged

### Install

#### From Cargo

`cargo install rust_ledger`

#### Binaries for Linux and macOS

We distribute binaries for the above platforms. See [releases](https://github.com/ebcrowder/rust_ledger/releases) for a
complete list.

#### Build from Source

Alternatively, clone this repo and do the following:

- If Rust is not installed on your machine, follow the instructions on how to do that
  here: https://www.rust-lang.org/tools/install
- run `cargo build --release` to compile the binary
- copy the `/target/release/rust_ledger` binary to `/usr/bin` or wherever your system maintains application binaries

## Test

- `cargo test`

### Usage

`rust_ledger --help` will provide a menu of all available commands and optional arguments.

```bash
rust_ledger <version>
Eric Crowder <eric@ebcrowder.dev>

USAGE:
    rust_ledger [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    account     account module
    balance     balance module
    budget      budget module
    csv         csv module
    help        Prints this message or the help of the given subcommand(s)
    register    register module
```

## rust_ledger `yaml` file format

In lieu of the plain text ledger file format, this project uses a defined `yaml` schema. `yaml` has a relatively simple
syntax and is able to represent useful data types, such as lists, quite easily.

An example ledger `yaml` file can be found at `examples/example.yaml`.

Further, parsing `yaml` via is trivial thanks to tools such as `serde`. This allowed me to skip writing a parser to
support the `ledger` plain text file format and focus on implementing functionality. Additionally, modern formatting
tools, such as [prettier](https://prettier.io) can be leveraged to maintain `yaml` files with ease as they are agnostic
to the schema of the underlying file.

The `ledger` tool has been rewritten in [many](https://github.com/ledger/ledger/wiki/Ports) programming languages.
Almost all of these ports utilize the original `ledger` plain text format. Above all else, I view `rust_ledger` as an
experiment in using a different ledger file format while maintaining the core feature set of the original tool.

`rust_ledger` utilizes `yaml` files in the following format:

```yaml
currency: USD

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

- the `currency` field should be a [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code.
- the `account` field should be expressed in the following format: `account_classification:account_name`.
- the `amount` field should be a number. It can include up to two (2) decimal points.
- the `date` field should be in the following format: `YYYY-MM-DD`.

## Transactions

Transactions can be expressed in two different ways. One is a "simplified" format for transactions that only impact two
accounts:

```yaml
- date: 2020-01-01
  amount: 200
  offset_account: liability:cc_amex
  description: grocery store
  account: expense:expense_general
```

The sign (debit / credit) associated with the `offset_account` value is the opposite of the sign of the value contained
in `amount` field.

In the above example transaction, since `expense_general` was debited by 200, the `cc_amex` account will be credited by
the same amount.

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
      account: liability:cc_amex
```

Transactions that only involve two accounts can also be expressed in the above format.

## Specifying the rust_ledger file path via environment variable

Optionally, the ledger file path can be set via the environment variable `RUST_LEDGER_FILE` in lieu of specifying
whenever the program is invoked. If `-f` is provided with a file path, the file provided will be used instead of
any `RUST_LEDGER_FILE` set.

## API

### account

Lists all accounts contained within the ledger file.

```bash
rust_ledger-account
account module

USAGE:
    rust_ledger account [OPTIONS] --filename <filename>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filename <filename>    location of ledger file
```

example output:

```
 Account 
----------------------------
 asset:cash_checking 
 asset:cash_savings 
 liability:cc_amex 
 equity:equity 
 expense:grocery 
 expense:general 
 expense:mortgage 
 income:general 
```

### balance

Lists account balances to date.

```bash
rust_ledger-balance
balance module

USAGE:
    rust_ledger balance [OPTIONS] --filename <filename>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filename <filename>    location of ledger file
```

example output:

```
 Account             | Balance 
---------------------+------------
 asset               |  
 asset:cash_checking | $-400.00 
  asset:cash_savings | $1,000.00 
 liability           |  
   liability:cc_amex | $-455.00 
 equity              |  
       equity:equity | $-3,500.00 
 expense             |  
     expense:grocery | $635.00 
     expense:general | $1,020.00 
    expense:mortgage | $2,000.00 
 income              |  
      income:general | $-300.00 
                     |  
 check               | 0 
```

### register

Lists general ledger transactions to date. The output can be filtered by any field via optional parameter.

```bash
rust_ledger-register
register module

USAGE:
    rust_ledger register [OPTIONS] --filename <filename>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filename <filename>    location of ledger file
    -g, --group <group>          group register output by value
    -o, --option <option>        filter output by optional value
```

- register report can be optionally rolled up via the `group` parameter (`yearly`, `monthly` or `daily`)
- if a `group` parameter is specified, a `option` parameter must also be specified to indicate the value to group by.
  For example, this value could be `2020` if using a `yearly` group parameter or `12` (December) if using a `monthly`
  group parameter.
- register report can also be optionally filtered by `option` parameter. All matching `Description`, `Account`
  or `Amount` values will be included in the output.

example output:

```
 Date       | Description        | Account             | Amount 
------------+--------------------+---------------------+------------
 2019-12-31 | weekly groceries   | expense:grocery     | $455.00 
 2019-12-31 | weekly groceries   | liability:cc_amex   | $-455.00 
 2020-01-01 | mortage            | expense:mortgage    | $2,000.00 
 2020-01-01 | mortage            | asset:cash_checking | $-2,000.00 
 2020-01-01 | stuff              | expense:general     | $1,000.00 
 2020-01-01 | stuff              | asset:cash_savings  | $-1,000.00 
 2020-01-01 | grocery store      | expense:general     | $20.00 
 2020-01-01 | grocery store      | expense:grocery     | $180.00 
 2020-01-01 | grocery store      | asset:cash_checking | $-200.00 
 2020-01-01 | donut sale to dale | asset:cash_checking | $300.00 
 2020-01-01 | donut sale to dale | income:general      | $-300.00 
```

example output for `rust_ledger -f RUST_LEDGER_FILE -o grocery`:

```bash
 Date       | Description      | Account             | Amount 
------------+------------------+---------------------+----------
 2019-12-31 | weekly groceries | expense:grocery     | $455.00 
 2020-01-01 | grocery store    | expense:general     | $20.00 
 2020-01-01 | grocery store    | expense:grocery     | $180.00 
 2020-01-01 | grocery store    | asset:cash_checking | -$200.00
```

example output for `rust_ledger -f RUST_LEDGER_FILE -g yearly -o 2020`:

```bash
Date / Account      | Total
---------------------+------------
2020                |  
expense:grocery     | $180.00
expense:mortgage    | $2,000.00
asset:cash_checking | -$1,900.00
income:general      | -$300.00
asset:cash_savings  | -$1,000.00
expense:general     | $1,020.00
```

### budget

Outputs a report of budgeted and actual values for income statement accounts.

```bash
rust_ledger-budget 
budget module

USAGE:
    rust_ledger budget --filename <filename> --group <group> --option <option>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filename <filename>    location of ledger file
    -g, --group <group>          group budget output by value
    -o, --option <option>        filter output by optional value
```

- budget report is rolled up via the `group` parameter (`yearly`, `monthly` or `daily`)
- report is filtered by `option` parameter. For example, this value could be `2020`
  if using a `yearly` group parameter or `12` (December) if using a `monthly` group parameter.

Here is an example output of `rust_ledger budget -f RUST_LEDGER_FILE -g yearly -o 2020`:

```
 Date / Account   | Budget     | Actual    | Delta 
------------------+------------+-----------+------------
 2020             |            |           |  
 income:general   | $0.00      | -$300.00  | $300.00 
 expense:mortgage | $24,000.00 | $2,000.00 | $22,000.00 
 expense:grocery  | $6,000.00  | $180.00   | $5,820.00 
 expense:general  | $0.00      | $1,020.00 | -$1,020.00 
```

### csv

```bash
rust_ledger-csv
csv module

USAGE:
    rust_ledger csv [OPTIONS] --csv <csv> --filename <filename>

FLAGS:
    -h, --help       Prints help information
    -i, --invert     invert amount for each csv transaction
    -V, --version    Prints version information

OPTIONS:
    -c, --csv <csv>              path of csv file
    -f, --filename <filename>    location of ledger file
    -o, --offset <offset>        offset account for each csv transaction
```

Converts `csv` files to `yaml` format expected by `rust_ledger`:

- should be invoked with `-f` and `-c` arguments. These include the rust_ledger file location (unless specified via
  environment variable), csv file location and account offset, respectively.
- the `-i` flag can be used to invert the sign of the "amount" column values that are being imported. This is useful
  when working with CSV files that represent debits as negative values and credits as positive values.
- the account offset (`-o` argument) would be the offset transaction that the csv transactions should be posted against.
- the csv tool will look for existing transactions that have matching `description` fields and will populate the
  appropriate expense/income accounts for any matches. Non-matches will use a default of `expense:general`
  or `income:general`, which is determined based on the sign of the `amount` field (or `debit` or `credit`) contained in
  the transaction.

The CSV tool can import columns with the following case-sensitive names:

- date
- description
- name
- amount
- debit
- credit

Often, banks will provide exports in one of two formats: 1) amounts are represented in one column whereby debits and
credits are identified by negative and positive (or vice-versa) amounts or 2) separate debit and credit columns. The CSV
import tool can handle both scenarios.

CSV file(s) should have `date`, `description` and `name` columns as they are required fields.