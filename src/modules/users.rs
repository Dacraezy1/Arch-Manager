use crate::cli::UsersCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: UsersCmd) -> AppResult<()> {
    match cmd {
        UsersCmd::Add { user } => run("useradd", &["-m", &user]),
        UsersCmd::Del { user } => run("userdel", &["-r", &user]),
        UsersCmd::Passwd { user } => run("passwd", &[&user]),
        UsersCmd::List => run("cut", &["-d:", "-f1", "/etc/passwd"]),
    }
}
