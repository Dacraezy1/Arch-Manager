use crate::cli::DisksCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: DisksCmd) -> AppResult<String> {
    match cmd {
        DisksCmd::List => run("lsblk", &["-f"]),
        DisksCmd::Mounts => run("findmnt", &[]),
    }
}
