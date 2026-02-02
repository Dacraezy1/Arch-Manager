use crate::cli::SystemdCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: SystemdCmd) -> AppResult<()> {
    match cmd {
        SystemdCmd::Status { service } => run("systemctl", &["status", &service]),
        SystemdCmd::Start { service } => run("systemctl", &["start", &service]),
        SystemdCmd::Stop { service } => run("systemctl", &["stop", &service]),
        SystemdCmd::Restart { service } => run("systemctl", &["restart", &service]),
        SystemdCmd::Enable { service } => run("systemctl", &["enable", &service]),
        SystemdCmd::Disable { service } => run("systemctl", &["disable", &service]),
        SystemdCmd::List => run("systemctl", &["list-units", "--type=service"]),
    }
}
