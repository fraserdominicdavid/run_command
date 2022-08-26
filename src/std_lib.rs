use std::{
    io,
    process::{Child, Command, Stdio},
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
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .output();
    CommandOutput::from(output)
}
