![](https://github.com/ebcrowder/rust-ledger/workflows/Rust/badge.svg)

# rust_ledger

### this project is for learning purposes only.

You should _definitely_ use the original (https://github.com/ledger/ledger) or one of its ports (https://github.com/ledger/ledger/wiki/Ports) which actually work and have more features.

As a former CPA, I could not resist building my own accounting system.

### Summary

- Spiritual port of ledger
- Smaller feature set based on my particular use cases - primarily, _1)_ how much money have I spent during `xyz` time period and _2)_ what is my financial situation as of right now?
- Uses `yaml` files as data store
- Includes a tool to convert `csv` files to `yaml` format

### API

- accounts
  - lists accounts
  - example output:

```
account              | account_type
checking             | asset
savings              | asset
credit_card          | liability
mortgage             | liability
auto                 | liability
equity               | equity
expense_credit_card  | expense
```

- balance
  - lists account balances to date
  - example output:

```
account_type         | account              | balance
asset                | checking             | 1,500
asset                | savings              | 2,000
liability            | credit_card          | -50
liability            | mortgage             | -100,000
liability            | auto                 | -5,000
equity               | equity               | 101,500
expense              | expense_credit_card  | 50
check                | 0
```

- register
  - lists general ledger transactions to date
  - example output:

```
date      | debit | acct_name  | acct_offset_name
10/1/2019 | 98    | expense_cc | credit_card
10/1/2019 | 1     | expense_cc | credit_card
10/2/2019 | 49    | expense_cc | credit_card
10/3/2019 | 9     | expense_cc | credit_card
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
  - id: 2
    acct_name: expense-test-acct
    acct_type: expense
    debit_credit: 0

transactions:
  - acct_name: expense-test-acct
    date: "2019-01-01"
    debit_credit: 1
    acct_type: expense
    acct_offset_name: checking
  - acct_name: expense-test-acct
    date: "2019-01-02"
    debit_credit: 400
    acct_type: expense
    acct_offset_name: checking
```
