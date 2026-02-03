use crate::cli::NetworkCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: NetworkCmd) -> AppResult<String> {
    match cmd {
        NetworkCmd::List => run("nmcli", &["device", "status"]),
        NetworkCmd::Up { iface } => run("nmcli", &["device", "connect", &iface]),
        NetworkCmd::Down { iface } => run("nmcli", &["device", "disconnect", &iface]),
        NetworkCmd::WifiScan => run("nmcli", &["device", "wifi", "list"]),
    }
}
