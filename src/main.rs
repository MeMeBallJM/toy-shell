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

    return command.trim_end().to_string();
}

fn parse_command(command: &String) -> (Option<&str>, Vec<&str>) {
    let mut split = command.split_whitespace();

    if let Some(cmd_name) = split.next() {
        return (Some(cmd_name), split.collect());
    } else {
        return (None, vec![]);
    }
}

fn handle_command(cmd_name: Option<&str>, args: Vec<&str>) {
    let Some(cmd_name) = cmd_name else {
        return;
    };

    if cmd_name == "exit" {
        process::exit(0)
    }

    if cmd_name == "echo" {
        println!("{}", args.join(" "));
        return;
    }

    println!("{} {}: command not found", cmd_name, args.join(" "));
}

fn main() {
    let stdin = io::stdin();

    loop {
        display_prompt();
        let command = read_command(&stdin);
        let (cmd_name, args) = parse_command(&command);
        handle_command(cmd_name, args);
    }
}
