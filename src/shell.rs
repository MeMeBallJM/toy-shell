use core::panic;
use std::collections::HashMap;
use std::io::Write;
use std::io::{self, Stdin, Stdout};

type Bulltin = fn(&[&str], &Shell);

pub struct Shell {
    bulltin_cmds: HashMap<String, Bulltin>,
    input: Stdin,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            bulltin_cmds: HashMap::new(),
            input: io::stdin(),
        }
    }

    pub fn add_bulltin(&mut self, cmd_name: &str, func: Bulltin) {
        self.bulltin_cmds.insert(cmd_name.to_string(), func);
    }

    pub fn get_bulltins(&self) -> Vec<&str> {
        self.bulltin_cmds.keys().map(|s| s.as_str()).collect()
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

        if let Some(bulltin_cmd) = self.bulltin_cmds.get(args[0]) {
            bulltin_cmd(&args, &self);
            return;
        }

        // TODO: Add program execution

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
