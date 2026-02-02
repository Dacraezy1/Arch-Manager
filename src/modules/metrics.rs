use crate::cli::MetricsCmd;
use crate::error::AppResult;
use crate::utils::cmd::{output, require_cmd, run};
use crate::utils::output::{info, warn};
use std::fs;

pub fn handle(cmd: MetricsCmd) -> AppResult<()> {
    match cmd {
        MetricsCmd::All => all(),
        MetricsCmd::Ram => ram(),
        MetricsCmd::CpuTemp => cpu_temp(),
        MetricsCmd::Gpu => gpu(),
        MetricsCmd::Battery => battery(),
        MetricsCmd::Load => load(),
        MetricsCmd::Uptime => uptime(),
    }
}

fn all() -> AppResult<()> {
    println!("RAM: {}", ram_value()?);
    println!("CPU_TEMP: {}", cpu_temp_value()?);
    println!("GPU: {}", gpu_value()?);
    println!("BATTERY: {}", battery_value()?);
    println!("LOAD: {}", load_value()?);
    println!("UPTIME: {}", uptime_value()?);
    Ok(())
}

fn ram() -> AppResult<()> {
    info("RAM usage");
    run("free", &["-h"])
}

fn ram_value() -> AppResult<String> {
    let out = output("free", &["-h"])?;
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if line.starts_with("Mem:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                return Ok(format!("{}/{}", parts[2], parts[1]));
            }
        }
    }
    Ok("unknown".to_string())
}

fn cpu_temp() -> AppResult<()> {
    info("CPU temperature");
    require_cmd("sensors")?;
    run("sensors", &[])
}

fn cpu_temp_value() -> AppResult<String> {
    if require_cmd("sensors").is_err() {
        return Ok("sensors not installed".to_string());
    }
    let out = output("sensors", &[])?;
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if line.contains("Package id") || line.contains("Tctl") || line.contains("CPU Temp") {
            return Ok(line.trim().to_string());
        }
    }
    Ok("unknown".to_string())
}

fn gpu() -> AppResult<()> {
    info("GPU usage");
    if require_cmd("nvidia-smi").is_ok() {
        run("nvidia-smi", &["--query-gpu=utilization.gpu,temperature.gpu", "--format=csv,noheader"])
    } else if require_cmd("rocm-smi").is_ok() {
        run("rocm-smi", &["--showuse"])
    } else {
        warn("No supported GPU monitor found (nvidia-smi or rocm-smi).");
        Ok(())
    }
}

fn gpu_value() -> AppResult<String> {
    if require_cmd("nvidia-smi").is_ok() {
        let out = output(
            "nvidia-smi",
            &["--query-gpu=utilization.gpu,temperature.gpu", "--format=csv,noheader"],
        )?;
        let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !text.is_empty() {
            return Ok(text);
        }
    }
    if require_cmd("rocm-smi").is_ok() {
        let out = output("rocm-smi", &["--showuse"])?;
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if line.contains("GPU") && line.contains("%") {
                return Ok(line.trim().to_string());
            }
        }
    }
    if let Ok(v) = fs::read_to_string("/sys/class/drm/card0/device/gpu_busy_percent") {
        let val = v.trim();
        if !val.is_empty() {
            return Ok(format!("{}%", val));
        }
    }
    Ok("unavailable".to_string())
}

fn battery() -> AppResult<()> {
    info("Battery health");
    if require_cmd("upower").is_err() {
        warn("upower not installed.");
        return Ok(());
    }

    let out = output("upower", &["-e"])?;
    let text = String::from_utf8_lossy(&out.stdout);
    let device = text.lines().find(|l| l.contains("battery"));

    if let Some(dev) = device {
        run("upower", &["-i", dev])
    } else {
        warn("No battery detected.");
        Ok(())
    }
}

fn battery_value() -> AppResult<String> {
    if require_cmd("upower").is_err() {
        return Ok("upower not installed".to_string());
    }

    let out = output("upower", &["-e"])?;
    let text = String::from_utf8_lossy(&out.stdout);
    let device = match text.lines().find(|l| l.contains("battery")) {
        Some(d) => d.to_string(),
        None => return Ok("no battery".to_string()),
    };

    let out = output("upower", &["-i", &device])?;
    let text = String::from_utf8_lossy(&out.stdout);
    let mut percent = "";
    let mut capacity = "";
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with("percentage:") {
            percent = l.trim_start_matches("percentage:").trim();
        }
        if l.starts_with("capacity:") {
            capacity = l.trim_start_matches("capacity:").trim();
        }
    }

    if !percent.is_empty() || !capacity.is_empty() {
        return Ok(format!("{} {}", percent, capacity).trim().to_string());
    }

    Ok("unknown".to_string())
}

fn load() -> AppResult<()> {
    info("System load");
    run("uptime", &[])
}

fn load_value() -> AppResult<String> {
    if let Ok(v) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = v.split_whitespace().collect();
        if parts.len() >= 3 {
            return Ok(format!("{} {} {}", parts[0], parts[1], parts[2]));
        }
    }
    Ok("unknown".to_string())
}

fn uptime() -> AppResult<()> {
    info("Uptime");
    run("uptime", &["-p"])
}

fn uptime_value() -> AppResult<String> {
    let out = output("uptime", &["-p"])?;
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if text.is_empty() {
        Ok("unknown".to_string())
    } else {
        Ok(text)
    }
}
