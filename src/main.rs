mod shell;
use shell::Shell;

fn exit(_: &[&str], _shell: &Shell) {
    std::process::exit(0);
}

fn echo(args: &[&str], _shell: &Shell) {
    print!("{}\n", args[1..].join(" "));
}

fn r#type(args: &[&str], shell: &Shell) {
    'args: for &arg in &args[1..] {
        for bulltin in shell.get_builtins() {
            if bulltin == arg {
                println!("{arg} is a shell builtin");
                continue 'args;
            }
        }

        let path = Shell::search_path(arg);

        if let Some(path) = path {
            println!("{arg} is {path}");
        } else {
            println!("{arg} not found");
        }
    }
}

fn pwd(_args: &[&str], _shell: &Shell) {
    let path = std::env::current_dir().expect("Can't pwd");
    println!("{}", path.display());
}

fn main() {
    let mut shell = Shell::new();
    shell.add_builtin("exit", exit);
    shell.add_builtin("echo", echo);
    shell.add_builtin("type", r#type);
    shell.add_builtin("pwd", pwd);

    shell.start();
}
