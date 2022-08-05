use tokio::{
    io,
    process::{Child, Command},
};

use crate::CommandOutput;

pub async fn async_run_command(command: &str) -> CommandOutput {
    let output = Command::new("sh").arg("-c").arg(command).output().await;
    CommandOutput::from(output)
}

pub async fn async_spawn_command(command: &str) -> io::Result<Child> {
    Command::new("sh").arg("-c").arg(command).spawn()
}
