use std::io;

use crate::models::Transaction;

pub fn check_balance(balance: &i64) {
    println!("Your balance is: ${:.2}", *balance as f64 / 100.0);
}

pub fn deposit(balance: &mut i64, transactions: &mut Vec<Transaction>) {
    println!("Please select the amount to deposit");
    let mut amount: String = String::new();
    io::stdin().read_line(&mut amount).expect("Invalid value");

    match amount.trim().parse::<f64>() {
        Ok(num) => {
            if num > 0.0 {
                let cents = (num * 100.0).round() as i64;
                *balance += cents;
                let transaction = Transaction::new("DEPOSIT".to_string(), cents);
                transactions.push(transaction);
                println!(
                    "You have successfully deposited ${:.2}, and your new balance is ${:.2}",
                    num,
                    *balance as f64 / 100.0
                );
            } else {
                println!("The amount needs to be greater than 0");
            }
        }
        Err(_) => eprintln!("Invalid value"),
    };
}

pub fn withdraw(balance: &mut i64, transactions: &mut Vec<Transaction>) {
    println!("Select the amount to withdraw");
    let mut amount: String = String::new();

    io::stdin().read_line(&mut amount).expect("Invalid value");

    match amount.trim().parse::<f64>() {
        Ok(num) => {
            if num <= 0.0 {
                println!("The amount should be greater than 0");
            } else {
                let cents = (num * 100.0).round() as i64;
                if cents > *balance {
                    println!(
                        "You don't have enough money, your current balance is: ${:.2}",
                        *balance as f64 / 100.0
                    );
                } else {
                    *balance -= cents;
                    let transaction = Transaction::new("WITHDRAW".to_string(), cents);
                    transactions.push(transaction);
                    println!(
                        "You have withdrawn ${:.2}, and your new balance is: ${:.2}",
                        num,
                        *balance as f64 / 100.0
                    );
                }
            }
        }
        Err(_) => eprintln!("Invalid value"),
    };
}

pub fn list_transactions(transactions: &mut Vec<Transaction>) {
    if transactions.is_empty() {
        println!("You haven't transactions yet");
    } else {
        println!("Your transactions are:");
        for t in transactions {
            println!(
                "ID: {}, Type: {}, Amount: ${:.2}, Time: {}",
                t.id,
                t.ttype,
                t.amount as f64 / 100.0,
                t.created_on
            );
        }
    }
}
