use crate::cli::UpdatesCmd;
use crate::error::AppResult;
use crate::utils::cmd::{output, run};
use crate::utils::output::{info, ok, warn};

pub fn handle(cmd: UpdatesCmd) -> AppResult<()> {
    match cmd {
        UpdatesCmd::Check => check_updates(false),
        UpdatesCmd::List => check_updates(true),
    }
}

pub fn check_updates(list: bool) -> AppResult<()> {
    run("pacman", &["-Sy"])?;

    let out = output("checkupdates", &[])?;
    if !out.status.success() {
        warn("No updates or checkupdates failed.");
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        ok("System is up to date.");
        return Ok(());
    }

    info(&format!("Updates available: {}", lines.len()));
    if list {
        for line in lines {
            println!("{line}");
        }
    }

    Ok(())
}
