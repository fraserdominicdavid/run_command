use std::{
    io,
    os::unix::process::ExitStatusExt,
    process::{Command, ExitStatus, Output},
};

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    fn from(output: io::Result<Output>) -> CommandOutput {
        match output {
            Ok(output) => CommandOutput {
                status: output.status,
                stdout: String::from_utf8(output.stdout).unwrap_or("failed".to_string()),
                stderr: String::from_utf8(output.stderr).unwrap_or("failed".to_string()),
            },
            Err(e) => CommandOutput {
                status: ExitStatus::from_raw(1),
                stdout: "".to_string(),
                stderr: format!("{e:#?}"),
            },
        }
    }

    pub fn success(&self) -> bool {
        self.status.success()
    }
}

pub fn run_command(command: &str) -> CommandOutput {
    let output = Command::new("sh").arg("-c").arg(command).output();
    CommandOutput::from(output)
}
