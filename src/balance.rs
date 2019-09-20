// returns balances of all general ledger accounts

// use std::fs;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn balance(filename: &str) -> io::Result<()> {
    // let file_string = fs::read_to_string(filename).expect("Unable to read ledger file");

    // // split words by separate lines and collect them into a Vector
    // let mut ledger_vec: Vec<&str> = file_string.lines().collect();

    // for line in ledger_vec {
    //     println!("{:?}", line)

    //   line.contains()

    // }

    let mut file = File::open(filename).unwrap(); // fail if the file doesn't exist
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
