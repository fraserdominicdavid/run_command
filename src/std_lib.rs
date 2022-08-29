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

pub fn run_command_pipe_to_terminal(command: &str) -> CommandOutput {
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn();
    match child {
        Ok(child) => {
            let output = child.wait_with_output();
            CommandOutput::from(output)
        },
        Err(e) => {
            CommandOutput::from(Err(e))
        }
    }
}
