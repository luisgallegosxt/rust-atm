# Rust ATM Simulator

## Description

A simple command-line application written in Rust that simulates an ATM machine. Users can perform basic banking operations such as checking their account balance, depositing money, withdrawing money, and viewing their transaction history. The application logs each transaction with a unique ID and timestamp.

## Features

- **Check Balance**: View the current account balance.
- **Deposit**: Add funds to the account.
- **Withdraw**: Remove funds from the account, with checks for sufficient balance.
- **List Transactions**: Display all transactions with details including ID, type, amount, and creation time.
- **Exit**: Quit the application.

## Requirements

- Rust programming language (edition 2024 or compatible)
- Cargo package manager

## Dependencies

- `chrono` - For handling dates and times in transactions.
- `uuid` - For generating unique transaction IDs.

## Installation

1. Ensure Rust and Cargo are installed on your system. You can install them from [rustup.rs](https://rustup.rs/).
2. Clone or download this repository.
3. Navigate to the project directory (`rust-atm`).

## Usage

To run the application:

```bash
cargo run
```

The application will start and present a menu with options. Enter the corresponding number to select an operation.

## Example Interaction

```
Welcome to the Rust ATM
Please select the required operation:
1. Check Balance
2. Deposit
3. Withdraw
4. List transactions
5. Exit
```

Select an option and follow the prompts.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

Luis Gallegos
