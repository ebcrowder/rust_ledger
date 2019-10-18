# rust_ledger

### this project is for learning purposes - it is far from complete.

You should _definitely_ use the original (https://github.com/ledger/ledger) or one of its ports (https://github.com/ledger/ledger/wiki/Ports) which actually work.

### Summary
- Spiritual port of ledger 
- Uses `yaml` files as data store of accounts and transactions
- Includes a tool to convert `csv` files to `yaml` format 

### API
- Accounts
   - lists accounts 
- Balance
   - lists account balances to date
- Register
   - lists general ledger transactions to date

### rust_ledger file format
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
    desc: test
    acct_offset_name: checking 
  - acct_name: expense-test-acct
    date: "2019-01-02"
    debit_credit: 400
    desc: test again
    acct_offset_name: checking 
```
