use core::panic;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::io::{self, Stdin};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;

type Builtin = fn(&[&str], &Shell);

pub struct Shell {
    builtin_cmds: HashMap<String, Builtin>,
    input: Stdin,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            builtin_cmds: HashMap::new(),
            input: io::stdin(),
        }
    }

    pub fn add_builtin(&mut self, cmd_name: &str, func: Builtin) {
        self.builtin_cmds.insert(cmd_name.to_string(), func);
    }

    pub fn get_builtins(&self) -> Vec<&str> {
        self.builtin_cmds.keys().map(|s| s.as_str()).collect()
    }

    pub fn search_path(name: &str) -> Option<String> {
        let path = std::env::var("PATH").unwrap_or(String::new());

        for dir in path.split(":") {
            let path = format!("{dir}/{name}");
            let Ok(metadata) = fs::metadata(path.as_str()) else {
                continue;
            };

            let permissions = metadata.permissions();
            let mode = permissions.mode();

            // Bitflag check if executable
            if mode & 0o100 != 0 {
                return Some(path);
            }
        }

        return None;
    }

    fn new_prompt(&self) -> Result<(), io::Error> {
        print!("$ ");
        std::io::stdout().flush()
    }

    fn read_command(&self) -> Result<String, io::Error> {
        let mut line = String::new();
        self.input.read_line(&mut line)?;

        Ok(line)
    }

    fn execute_cmd(&self, line: &str) {
        let args: Vec<_> = line.split_whitespace().collect();

        if args.is_empty() {
            return;
        }

        if let Some(bulltin_cmd) = self.builtin_cmds.get(args[0]) {
            bulltin_cmd(&args, &self);
            return;
        }

        if let Some(program_path) = Shell::search_path(args[0]) {
            let mut child = std::process::Command::new(program_path)
                .args(&args[1..])
                .arg0(args[0])
                .spawn()
                .expect("Couldn't spawn program");
            child.wait().expect("program failed to start");

            return;
        }

        print!("{}: command not found\n", args[0]);
    }

    pub fn start(self) {
        loop {
            if let Err(_) = self.new_prompt() {
                panic!("Couldn't display prompt");
            }

            let Ok(cmd) = self.read_command() else {
                panic!("Couldn't read command");
            };

            self.execute_cmd(cmd.as_ref());
        }
    }
}
