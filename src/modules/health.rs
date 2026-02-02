use crate::cli::HealthCmd;
use crate::error::AppResult;
use crate::modules::{news, updates};
use crate::utils::cmd::{output, run};
use crate::utils::output::{info, ok, warn};

pub fn handle(cmd: HealthCmd) -> AppResult<()> {
    match cmd {
        HealthCmd::Summary => summary(),
        HealthCmd::Full => full(),
        HealthCmd::Services => services(),
        HealthCmd::Disk => disk(),
        HealthCmd::Memory => memory(),
        HealthCmd::Cpu => cpu(),
        HealthCmd::Kernel => kernel(),
    }
}

fn summary() -> AppResult<()> {
    info("System Health Summary");
    kernel()?;
    disk()?;
    memory()?;
    services()?;
    Ok(())
}

fn full() -> AppResult<()> {
    summary()?;
    info("Package Updates");
    updates::check_updates(false)?;
    info("Arch News");
    news::latest()?;
    Ok(())
}

fn services() -> AppResult<()> {
    info("Failed services");
    let out = output("systemctl", &["--failed"])?;
    if out.status.success() {
        let text = String::from_utf8_lossy(&out.stdout);
        if text.lines().count() <= 1 {
            ok("No failed services.");
        } else {
            println!("{text}");
        }
    } else {
        warn("systemctl --failed returned error.");
    }
    Ok(())
}

fn disk() -> AppResult<()> {
    info("Disk usage");
    run("df", &["-h"])
}

fn memory() -> AppResult<()> {
    info("Memory usage");
    run("free", &["-h"])
}

fn cpu() -> AppResult<()> {
    info("CPU info");
    run("lscpu", &[])
}

fn kernel() -> AppResult<()> {
    info("Kernel");
    run("uname", &["-a"])
}
