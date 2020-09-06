use std::process::{Command, Child};
use crate::error::{CoreError, Result};

#[derive(Debug)]
pub struct Action {
    pub ty: ActionType,
    pub command: Command,
}

impl Action {
    pub fn new(command: String, ty: ActionType) -> Action {
        Action {
            ty,
            command: Command::new(command),
        }
    }

    pub fn args(&mut self, args: Vec<String>) {
        self.command.args(args);
    }

    pub fn run(&mut self) -> Result<Child> {
        match self.command.spawn() {
            Ok(c) => return Ok(c),
            Err(e) => return Err(CoreError::from(e)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ActionType {
    Clicked,
    MouseHover,
}