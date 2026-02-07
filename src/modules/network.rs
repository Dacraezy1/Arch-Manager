use crate::cli::NetworkCmd;
use crate::error::AppResult;
use crate::utils::cmd::{output, require_cmd, run};

pub fn handle(cmd: NetworkCmd) -> AppResult<String> {
    match cmd {
        NetworkCmd::List => list_devices(),
        NetworkCmd::Up { iface } => connect_iface(&iface),
        NetworkCmd::Down { iface } => disconnect_iface(&iface),
        NetworkCmd::WifiScan => wifi_scan(),
    }
}

fn list_devices() -> AppResult<String> {
    if require_cmd("nmcli").is_ok() {
        return command_output("nmcli", &["device", "status"]);
    }
    command_output("ip", &["link", "show"])
}

fn connect_iface(iface: &str) -> AppResult<String> {
    if require_cmd("nmcli").is_ok() {
        return command_output("nmcli", &["device", "connect", iface]);
    }
    if require_cmd("ip").is_ok() {
        run("ip", &["link", "set", iface, "up"])?;
        return Ok(format!("Brought {iface} up.\n"));
    }
    Ok("No supported network manager found.\n".to_string())
}

fn disconnect_iface(iface: &str) -> AppResult<String> {
    if require_cmd("nmcli").is_ok() {
        return command_output("nmcli", &["device", "disconnect", iface]);
    }
    if require_cmd("ip").is_ok() {
        run("ip", &["link", "set", iface, "down"])?;
        return Ok(format!("Brought {iface} down.\n"));
    }
    Ok("No supported network manager found.\n".to_string())
}

fn wifi_scan() -> AppResult<String> {
    if require_cmd("nmcli").is_ok() {
        return command_output("nmcli", &["device", "wifi", "list"]);
    }
    if require_cmd("iw").is_ok() {
        return command_output("iw", &["dev", "wlan0", "scan"]);
    }
    Ok("Wi-Fi scan not supported (nmcli/iw missing).\n".to_string())
}

fn command_output(cmd: &str, args: &[&str]) -> AppResult<String> {
    let out = output(cmd, args)?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(crate::error::AppError::CommandFailed(format!(
            "{cmd} {:?}",
            args
        )))
    }
}
