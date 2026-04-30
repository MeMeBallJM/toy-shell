use std::io::Write;
use std::io::{self, Stdin};
use std::process;

fn display_prompt() {
    print!("$ ");
    io::stdout().flush().expect("Unable to flush prompt");
}

fn read_command(stdin: &Stdin) -> String {
    let mut command = String::new();

    if let Err(error) = stdin.read_line(&mut command) {
        println!("Unable to read command {}", error);
    }

    command.trim_end().to_owned()
}

fn handle_command(command: &String) {
    match command.as_str() {
        "exit" => process::exit(0),
        _ => println!("{}: command not found", command),
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        display_prompt();
        let command = read_command(&stdin);
        handle_command(&command);
    }
}
