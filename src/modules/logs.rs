use crate::cli::LogsCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: LogsCmd) -> AppResult<()> {
    match cmd {
        LogsCmd::Tail { lines } => run("journalctl", &["-n", &lines.to_string()]),
        LogsCmd::Service { service } => run("journalctl", &["-u", &service]),
        LogsCmd::Boot => run("journalctl", &["-b"]),
    }
}
