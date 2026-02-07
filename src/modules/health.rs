use crate::cli::HealthCmd;
use crate::error::AppResult;
use crate::modules::{news, updates};
use crate::utils::cmd::{output, require_cmd, run};

pub fn handle(cmd: HealthCmd) -> AppResult<String> {
    let mut output = String::new();

    match cmd {
        HealthCmd::Summary => {
            output.push_str(&summary()?);
        }
        HealthCmd::Full => {
            output.push_str(&full()?);
        }
        HealthCmd::Services => {
            output.push_str(&services()?);
        }
        HealthCmd::Disk => {
            output.push_str(&disk()?);
        }
        HealthCmd::Memory => {
            output.push_str(&memory()?);
        }
        HealthCmd::Cpu => {
            output.push_str(&cpu()?);
        }
        HealthCmd::Kernel => {
            output.push_str(&kernel()?);
        }
        HealthCmd::Network => {
            output.push_str(&network()?);
        }
    }

    Ok(output)
}

fn summary() -> AppResult<String> {
    let mut out = String::new();
    out.push_str(&kernel()?);
    out.push_str(&disk()?);
    out.push_str(&memory()?);
    out.push_str(&services()?);
    out.push_str(&network()?);
    Ok(out)
}

fn full() -> AppResult<String> {
    let mut out = String::new();
    out.push_str(&summary()?);
    out.push_str("\nPackage Updates:\n");
    out.push_str(&updates::check_updates(false)?);
    out.push_str("\nArch News:\n");
    out.push_str(&news::latest()?);
    out.push_str("\nNetwork Health:\n");
    out.push_str(&network()?);
    Ok(out)
}

fn services() -> AppResult<String> {
    let out = crate::utils::cmd::output("systemctl", &["--failed"])?;
    if out.status.success() {
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        if text.lines().count() <= 1 {
            Ok("No failed services.\n".into())
        } else {
            Ok(format!("{text}\n"))
        }
    } else {
        Ok("systemctl --failed returned error.\n".into())
    }
}

fn disk() -> AppResult<String> {
    command_output("df", &["-h"])
}

fn memory() -> AppResult<String> {
    command_output("free", &["-h"])
}

fn cpu() -> AppResult<String> {
    command_output("lscpu", &[])
}

fn kernel() -> AppResult<String> {
    command_output("uname", &["-a"])
}

fn network() -> AppResult<String> {
    let mut out = String::new();
    if let Some(iface) = default_interface()? {
        out.push_str(&format!("Interface: {iface}\n"));
        if require_cmd("ip").is_ok() {
            out.push_str("IP:\n");
            out.push_str(&command_output("ip", &["-4", "addr", "show", "dev", &iface])?);
        }
        if let Ok(state) = std::fs::read_to_string(format!("/sys/class/net/{iface}/operstate")) {
            out.push_str(&format!("State: {}\n", state.trim()));
        }
        if let Ok(text) = std::fs::read_to_string("/etc/resolv.conf") {
            out.push_str("DNS:\n");
            for line in text.lines().filter(|l| l.trim_start().starts_with("nameserver")) {
                out.push_str(line);
                out.push('\n');
            }
        }
    } else {
        out.push_str("No active network interface detected.\n");
    }
    Ok(out)
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

fn default_interface() -> AppResult<Option<String>> {
    if let Ok(routes) = std::fs::read_to_string("/proc/net/route") {
        for line in routes.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "00000000" {
                return Ok(Some(parts[0].to_string()));
            }
        }
    }
    if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name != "lo" {
                return Ok(Some(name));
            }
        }
    }
    Ok(None)
}
