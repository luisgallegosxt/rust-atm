use std::io;

use crate::models::Transaction;
mod models;
mod operations;

fn main() {
    let mut account = models::Account { balance: 0.0 };
    let mut transactions: Vec<Transaction> = vec![];

    println!("Welcome to the Rust ATM");
    loop {
        println!(
            "Please select the required operation:
            1. Check Balance
            2. Deposit
            3. Withdraw
            4. List transactions
            5. Exit
            "
        );
        let mut option: String = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Was no possible to read the line");

        match option.trim() {
            "1" => operations::check_balance(&account.balance),
            "2" => operations::deposit(&mut account.balance, &mut transactions),
            "3" => operations::withdraw(&mut account.balance, &mut transactions),
            "4" => operations::list_transactions(&mut transactions),
            "5" => {
                println!("you have selected option 5, Good bye");
                break;
            }
            _ => println!("Invalid Option"),
        };
    }
}
