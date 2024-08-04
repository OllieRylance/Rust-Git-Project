use std::io::{self, Write};


mod presentation;
mod service;
mod data_access;
mod data_storage;
mod infrastructure;
mod tests;

fn main() {
    // Entry point of the application
    // Parse commands and route to appropriate functions

    loop {
        // Prompt the user for input
        print!("> ");
        io::stdout().flush().unwrap();

        // Read the input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim the input and split it into command and arguments
        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        // Use the handle_command(command: &str, args: &[&str]) function to handle user input commands
        presentation::commands::handle_command(command, &args);
    }
}