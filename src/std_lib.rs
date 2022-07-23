use std::{
    io,
    process::{Child, Command},
};

use crate::output::CommandOutput;

pub fn run_command(command: &str) -> CommandOutput {
    let output = Command::new("sh").arg("-c").arg(command).output();
    CommandOutput::from(output)
}

pub fn spawn_command(command: &str) -> io::Result<Child> {
    Command::new("sh").arg("-c").arg(command).spawn()
}
