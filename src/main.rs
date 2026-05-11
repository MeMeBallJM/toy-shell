mod shell;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use shell::Shell;

fn exit(_: &[&str], _shell: &Shell) {
    std::process::exit(0);
}

fn echo(args: &[&str], _shell: &Shell) {
    print!("{}\n", args[1..].join(" "));
}

fn r#type(args: &[&str], shell: &Shell) {
    'args: for &arg in &args[1..] {
        for bulltin in shell.get_bulltins() {
            if bulltin == arg {
                println!("{arg} is a shell bulltin");
                continue 'args;
            }
        }

        let path = std::env::var("PATH").unwrap_or(String::new());

        for dir in path.split(":") {
            let path = format!("{dir}/{arg}");
            let Ok(metadata) = fs::metadata(path.as_str()) else {
                continue;
            };

            let permissions = metadata.permissions();
            let mode = permissions.mode();

            // Bitflag check if executable
            if mode & 0o100 != 0 {
                println!("{path}");
                continue 'args;
            }
        }

        println!("{arg} not found");
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.add_bulltin("exit", exit);
    shell.add_bulltin("echo", echo);
    shell.add_bulltin("type", r#type);

    shell.start();
}
