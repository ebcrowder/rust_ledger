// returns all general ledger transactions

use std::fs;

pub fn register(filename: &str) -> Result<(), std::io::Error> {
    let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");
    let account_vec: Vec<&str> = file_string.split('\n').collect();
    #[derive(Debug)]

    struct Accounts {
        account: String,
        amount: f32,
    }

    let mut transactions_vec: Vec<Accounts> = Vec::new();

    // iterate through text file and push transactions into vector
    for line in account_vec {
        if line.contains(':') {
            let line_vec: Vec<&str> = line.split('\t').collect();
            let transaction: Vec<&str> = line_vec[0].trim().split_ascii_whitespace().collect();

            if transaction.len() > 1 {
                let account = transaction[0].to_string();
                let amount = transaction[1].parse::<f32>().unwrap();

                transactions_vec.push(Accounts { account, amount })
            }
        }
    }

    let transactions_sum: f32 = transactions_vec.iter().map(|s| s.amount).sum();

    for transaction in transactions_vec {
        // TODO make this a bit more dynamic
        // such as checking transactions against a command line arg
        if transaction.account.contains("Expenses") {
            println!("{:?}", transaction);
        } else {
            println!("bananas")
        }
    }

    println!("total {:.2}", transactions_sum);

    Ok(())
}
