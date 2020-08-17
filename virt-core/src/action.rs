use std::process::{Command, Child};
use std::io::Error;

#[derive(Debug)]
pub struct Action {
    pub command: Command,

    pub args: Option<Vec<String>>,
}

impl Action {
    pub fn new(command: String, args: Option<Vec<String>>) -> Action {
        Action {
            command: Command::new(command),
            args,
        }
    }

    pub fn run(&mut self) -> Result<Child, Error> {
        match &self.args {
            Some(a) => 
                return self.command.args(a).spawn(),
            None => 
                return self.command.spawn(),
        }
    }
}