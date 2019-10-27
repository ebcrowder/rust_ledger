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

- Accounts
  - lists accounts
- Balance
  - lists account balances to date
- Register
  - lists general ledger transactions to date
- CSV
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
    date: '2019-01-01'
    debit_credit: 1
    acct_type: expense
    acct_offset_name: checking
  - acct_name: expense-test-acct
    date: '2019-01-02'
    debit_credit: 400
    acct_type: expense
    acct_offset_name: checking
```
