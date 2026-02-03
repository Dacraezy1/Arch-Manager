use crate::cli::HealthCmd;
use crate::error::AppResult;
use crate::modules::{news, updates};
use crate::utils::cmd::run;

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
    }

    Ok(output)
}

fn summary() -> AppResult<String> {
    let mut out = String::new();
    out.push_str(&kernel()?);
    out.push_str(&disk()?);
    out.push_str(&memory()?);
    out.push_str(&services()?);
    Ok(out)
}

fn full() -> AppResult<String> {
    let mut out = String::new();
    out.push_str(&summary()?);
    out.push_str("\nPackage Updates:\n");
    out.push_str(&updates::check_updates(false)?);
    out.push_str("\nArch News:\n");
    out.push_str(&news::latest()?);
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
    run("df", &["-h"])
}

fn memory() -> AppResult<String> {
    run("free", &["-h"])
}

fn cpu() -> AppResult<String> {
    run("lscpu", &[])
}

fn kernel() -> AppResult<String> {
    run("uname", &["-a"])
}
