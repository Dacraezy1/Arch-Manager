use crate::error::{AppError, AppResult};
use std::process::{Command, Output};

pub fn run(cmd: &str, args: &[&str]) -> AppResult<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| AppError::CommandFailed(format!("{cmd}: {e}")))?;
    if !status.success() {
        return Err(AppError::CommandFailed(format!("{cmd} {:?}", args)));
    }
    Ok(())
}

pub fn output(cmd: &str, args: &[&str]) -> AppResult<Output> {
    Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| AppError::CommandFailed(format!("{cmd}: {e}")))
}

pub fn require_cmd(cmd: &str) -> AppResult<()> {
    if which::which(cmd).is_err() {
        return Err(AppError::MissingDependency(cmd.to_string()));
    }
    Ok(())
}
