use crate::cli::AurCmd;
use crate::error::AppResult;
use crate::utils::cmd::{require_cmd, run};

pub fn handle(cmd: AurCmd) -> AppResult<()> {
    require_cmd("yay")?;
    match cmd {
        AurCmd::Install { pkg } => run("yay", &["-S", &pkg]),
        AurCmd::Update => run("yay", &["-Syu"]),
    }
}
