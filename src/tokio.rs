use tokio::{
    io,
    process::{Child, Command},
};

use crate::CommandOutput;

#[cfg(unix)]
pub async fn async_run_command(command: &str) -> CommandOutput {
    let output = Command::new("sh").arg("-c").arg(command).output().await;
    CommandOutput::from(output)
}

#[cfg(windows)]
pub async fn async_run_command(command: &str) -> CommandOutput {
    let output = Command::new("cmd").arg("/C").arg(command).output().await;
    CommandOutput::from(output)
}

#[cfg(unix)]
pub async fn async_spawn_command(command: &str) -> io::Result<Child> {
    Command::new("sh").arg("-c").arg(command).spawn()
}

#[cfg(windows)]
pub async fn async_spawn_command(command: &str) -> io::Result<Child> {
    Command::new("cmd").arg("/C").arg(command).spawn()
}
