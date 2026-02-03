use crate::cli::AurCmd;
use crate::error::AppResult;
use crate::utils::cmd::{require_cmd, run};

pub fn handle(cmd: AurCmd) -> AppResult<String> {
    require_cmd("yay")?;

    match cmd {
        AurCmd::Install { pkg } => {
            let pkg = pkg.trim();
            if pkg.is_empty() {
                return Err(anyhow::anyhow!("Package name cannot be empty"));
            }
            run("yay", &["-S", pkg])
        }
        AurCmd::Update => run("yay", &["-Syu"]),
    }
}
