use std::io;

use crate::models::Transaction;

pub fn check_balance(balance: &f32) {
    println!("Your balance is: {}", balance);
}

pub fn deposit(balance: &mut f32, transactions: &mut Vec<Transaction>) {
    println!("Please select the amount to deposit");
    let mut amount: String = String::new();
    io::stdin().read_line(&mut amount).expect("Invalid value");

    match amount.trim().parse::<f32>() {
        Ok(num) => {
            if num > 0.0 {
                *balance += num;
                let transaction = Transaction::new("DEPOSIT".to_string(), num);
                transactions.push(transaction);
                println!(
                    "Your have successfully deposited {}, and your new balance is {}",
                    num, balance
                );
            } else {
                println!("The amount needs to be greater than 0");
            }
        }
        Err(_) => eprintln!("Invalid value"),
    };
}

pub fn withdraw(balance: &mut f32, transactions: &mut Vec<Transaction>) {
    println!("Select the amount to withdraw");
    let mut amount: String = String::new();

    io::stdin().read_line(&mut amount).expect("Invalid value");

    match amount.trim().parse::<f32>() {
        Ok(num) => {
            if num <= 0.0 {
                println!("The amount should be greater than 0");
            } else if num > *balance {
                println!(
                    "You don't have enough money, your current balance is: {}",
                    balance
                );
            } else {
                *balance -= num;
                let transaction = Transaction::new("WITHDRAW".to_string(), num);
                transactions.push(transaction);
                println!(
                    "You have withdrawn {}, and your new balance is: {}",
                    num, balance
                )
            }
        }
        Err(_) => eprintln!("Invalid value"),
    };
}

pub fn list_transactions(transactions: &mut Vec<Transaction>) {
    if transactions.is_empty() {
        println!("You haven't transactions yet");
    } else {
        println!("Your transactions are: {:#?}", transactions)
    }
}
