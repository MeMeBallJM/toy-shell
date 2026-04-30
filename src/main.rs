use std::io;
use std::io::Write;

fn main() {
    print!("$ ");
    io::stdout().flush().expect("Unable to flush prompt");

    let mut buffer = String::new();
    let stdin = io::stdin();

    if let Err(error) = stdin.read_line(&mut buffer) {
        println!("Unable to read line from stdin: {}", error);
        return;
    }

    let input = &buffer[0..buffer.len() - 1];

    println!("{}: command not found", input);
}
