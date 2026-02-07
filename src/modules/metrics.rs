use crate::cli::MetricsCmd;
use crate::error::AppResult;
use crate::utils::cmd::{output, require_cmd, run};
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn handle(cmd: MetricsCmd) -> AppResult<String> {
    match cmd {
        MetricsCmd::All => all(),
        MetricsCmd::Ram => ram(),
        MetricsCmd::CpuTemp => cpu_temp(),
        MetricsCmd::CpuUsage => cpu_usage(),
        MetricsCmd::Gpu => gpu(),
        MetricsCmd::GpuTemp => gpu_temp(),
        MetricsCmd::GpuUsage => gpu_usage(),
        MetricsCmd::Battery => battery(),
        MetricsCmd::BatteryHealth => battery_health(),
        MetricsCmd::Load => load(),
        MetricsCmd::MemUsage => mem_usage(),
        MetricsCmd::NetStatus => net_status(),
        MetricsCmd::NetIp => net_ip(),
        MetricsCmd::NetTraffic => net_traffic(),
        MetricsCmd::Orphans => orphans(),
        MetricsCmd::Processes => processes(),
        MetricsCmd::Uptime => uptime(),
    }
}

fn all() -> AppResult<String> {
    let mut out = String::new();
    out.push_str(&format!("RAM: {}\n", ram_value()?));
    out.push_str(&format!("CPU_TEMP: {}\n", cpu_temp_value()?));
    out.push_str(&format!("CPU_USAGE: {}\n", cpu_usage_value()?));
    out.push_str(&format!("GPU: {}\n", gpu_value()?));
    out.push_str(&format!("GPU_TEMP: {}\n", gpu_temp_value()?));
    out.push_str(&format!("GPU_USAGE: {}\n", gpu_usage_value()?));
    out.push_str(&format!("BATTERY: {}\n", battery_value()?));
    out.push_str(&format!("BATTERY_HEALTH: {}\n", battery_health_value()?));
    out.push_str(&format!("LOAD: {}\n", load_value()?));
    out.push_str(&format!("MEM_USAGE: {}\n", mem_usage_value()?));
    out.push_str(&format!("NET_STATUS: {}\n", net_status_value()?));
    out.push_str(&format!("NET_IP: {}\n", net_ip_value()?));
    out.push_str(&format!("NET_TRAFFIC: {}\n", net_traffic_value()?));
    out.push_str(&format!("ORPHANS: {}\n", orphans_value()?));
    out.push_str(&format!("PROCESSES: {}\n", processes_value()?));
    out.push_str(&format!("UPTIME: {}\n", uptime_value()?));
    Ok(out)
}

fn ram() -> AppResult<String> {
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

fn cpu_temp() -> AppResult<String> {
    if require_cmd("sensors").is_err() {
        return Ok("sensors not installed".to_string());
    }
    run("sensors", &[])
}

fn cpu_temp_value() -> AppResult<String> {
    if require_cmd("sensors").is_err() {
        return cpu_temp_sysfs();
    }
    let out = output("sensors", &[])?;
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if line.contains("Package id") || line.contains("Tctl") || line.contains("CPU Temp") {
            return Ok(line.trim().to_string());
        }
    }
    cpu_temp_sysfs()
}

fn cpu_temp_sysfs() -> AppResult<String> {
    if let Ok(entries) = fs::read_dir("/sys/class/thermal") {
        for entry in entries.flatten() {
            let path = entry.path().join("temp");
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(raw) = contents.trim().parse::<f32>() {
                    if raw > 0.0 {
                        return Ok(format!("{:.1}°C", raw / 1000.0));
                    }
                }
            }
        }
    }
    Ok("unknown".to_string())
}

fn cpu_usage() -> AppResult<String> {
    Ok(cpu_usage_value()?)
}

fn cpu_usage_value() -> AppResult<String> {
    let (total_a, idle_a) = read_cpu_stat()?;
    thread::sleep(Duration::from_millis(120));
    let (total_b, idle_b) = read_cpu_stat()?;
    let total_delta = total_b.saturating_sub(total_a);
    let idle_delta = idle_b.saturating_sub(idle_a);
    if total_delta == 0 {
        return Ok("unknown".to_string());
    }
    let usage = 100.0 * (1.0 - idle_delta as f64 / total_delta as f64);
    Ok(format!("{usage:.1}%"))
}

fn read_cpu_stat() -> AppResult<(u64, u64)> {
    let stat = fs::read_to_string("/proc/stat")?;
    let mut total = 0u64;
    let mut idle = 0u64;
    for line in stat.lines() {
        if line.starts_with("cpu ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                for part in parts.iter().skip(1) {
                    total += part.parse::<u64>().unwrap_or(0);
                }
                idle = parts[4].parse::<u64>().unwrap_or(0);
            }
            break;
        }
    }
    Ok((total, idle))
}

fn gpu() -> AppResult<String> {
    if require_cmd("nvidia-smi").is_ok() {
        run("nvidia-smi", &["--query-gpu=utilization.gpu,temperature.gpu", "--format=csv,noheader"])
    } else if require_cmd("rocm-smi").is_ok() {
        run("rocm-smi", &["--showuse"])
    } else {
        Ok("No supported GPU monitor found.".to_string())
    }
}

fn gpu_value() -> AppResult<String> {
    if require_cmd("nvidia-smi").is_ok() {
        let out = output("nvidia-smi", &["--query-gpu=name", "--format=csv,noheader"])?;
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

fn gpu_temp() -> AppResult<String> {
    Ok(gpu_temp_value()?)
}

fn gpu_temp_value() -> AppResult<String> {
    if require_cmd("nvidia-smi").is_ok() {
        let out = output(
            "nvidia-smi",
            &["--query-gpu=temperature.gpu", "--format=csv,noheader"],
        )?;
        let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !text.is_empty() {
            return Ok(format!("{text}°C"));
        }
    }
    if require_cmd("rocm-smi").is_ok() {
        let out = output("rocm-smi", &["--showtemp"])?;
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if line.contains("Temperature") {
                return Ok(line.trim().to_string());
            }
        }
    }
    if let Ok(entries) = fs::read_dir("/sys/class/drm/card0/device/hwmon") {
        for entry in entries.flatten() {
            let temp = entry.path().join("temp1_input");
            if let Ok(contents) = fs::read_to_string(temp) {
                if let Ok(raw) = contents.trim().parse::<f32>() {
                    if raw > 0.0 {
                        return Ok(format!("{:.1}°C", raw / 1000.0));
                    }
                }
            }
        }
    }
    Ok("unavailable".to_string())
}

fn gpu_usage() -> AppResult<String> {
    Ok(gpu_usage_value()?)
}

fn gpu_usage_value() -> AppResult<String> {
    if require_cmd("nvidia-smi").is_ok() {
        let out = output(
            "nvidia-smi",
            &["--query-gpu=utilization.gpu", "--format=csv,noheader"],
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
            return Ok(format!("{val}%"));
        }
    }
    Ok("unavailable".to_string())
}

fn battery() -> AppResult<String> {
    if require_cmd("upower").is_err() {
        return Ok("upower not installed".to_string());
    }

    let out = output("upower", &["-e"])?;
    let text = String::from_utf8_lossy(&out.stdout);
    let device = text.lines().find(|l| l.contains("battery"));

    if let Some(dev) = device {
        run("upower", &["-i", dev])
    } else {
        Ok("No battery detected.".to_string())
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

fn battery_health() -> AppResult<String> {
    Ok(battery_health_value()?)
}

fn battery_health_value() -> AppResult<String> {
    if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("BAT") {
                continue;
            }
            let base = entry.path();
            let full = read_power_value(&base, &["energy_full", "charge_full"])?;
            let design = read_power_value(&base, &["energy_full_design", "charge_full_design"])?;
            if let (Some(full), Some(design)) = (full, design) {
                if design > 0.0 {
                    return Ok(format!("{:.0}%", (full / design) * 100.0));
                }
            }
            if let Ok(capacity) = fs::read_to_string(base.join("capacity")) {
                let val = capacity.trim();
                if !val.is_empty() {
                    return Ok(format!("{val}%"));
                }
            }
        }
    }
    if require_cmd("upower").is_ok() {
        let out = output("upower", &["-e"])?;
        let text = String::from_utf8_lossy(&out.stdout);
        if let Some(device) = text.lines().find(|l| l.contains("battery")) {
            let out = output("upower", &["-i", device])?;
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let l = line.trim();
                if l.starts_with("capacity:") {
                    return Ok(l.trim_start_matches("capacity:").trim().to_string());
                }
            }
        }
    }
    Ok("unknown".to_string())
}

fn read_power_value(base: &std::path::Path, keys: &[&str]) -> AppResult<Option<f32>> {
    for key in keys {
        let path = base.join(key);
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(val) = contents.trim().parse::<f32>() {
                if val > 0.0 {
                    return Ok(Some(val));
                }
            }
        }
    }
    Ok(None)
}

fn load() -> AppResult<String> {
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

fn uptime() -> AppResult<String> {
    run("uptime", &["-p"])
}

fn mem_usage() -> AppResult<String> {
    Ok(mem_usage_value()?)
}

fn mem_usage_value() -> AppResult<String> {
    let text = fs::read_to_string("/proc/meminfo")?;
    let mut total = 0u64;
    let mut available = 0u64;
    for line in text.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        }
    }
    if total == 0 {
        return Ok("unknown".to_string());
    }
    let used = total.saturating_sub(available);
    let percent = (used as f64 / total as f64) * 100.0;
    Ok(format!("{:.1}% ({} / {} MiB)", percent, used / 1024, total / 1024))
}

fn processes() -> AppResult<String> {
    Ok(processes_value()?)
}

fn net_status() -> AppResult<String> {
    Ok(net_status_value()?)
}

fn net_status_value() -> AppResult<String> {
    if let Some(iface) = default_interface()? {
        let state_path = format!("/sys/class/net/{iface}/operstate");
        let state = fs::read_to_string(state_path).unwrap_or_else(|_| "unknown".into());
        let state = state.trim();
        return Ok(format!("{iface} ({state})"));
    }
    Ok("unknown".to_string())
}

fn net_ip() -> AppResult<String> {
    Ok(net_ip_value()?)
}

fn net_ip_value() -> AppResult<String> {
    let iface = match default_interface()? {
        Some(name) => name,
        None => return Ok("unknown".to_string()),
    };
    if require_cmd("ip").is_ok() {
        let out = output("ip", &["-4", "addr", "show", "dev", &iface])?;
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with("inet ") {
                if let Some(addr) = line.split_whitespace().nth(1) {
                    return Ok(addr.to_string());
                }
            }
        }
    }
    Ok("unknown".to_string())
}

fn net_traffic() -> AppResult<String> {
    Ok(net_traffic_value()?)
}

fn net_traffic_value() -> AppResult<String> {
    let iface = match default_interface()? {
        Some(name) => name,
        None => return Ok("unknown".to_string()),
    };
    let rx_path = format!("/sys/class/net/{iface}/statistics/rx_bytes");
    let tx_path = format!("/sys/class/net/{iface}/statistics/tx_bytes");
    let rx = read_u64(rx_path.as_ref()).unwrap_or(0);
    let tx = read_u64(tx_path.as_ref()).unwrap_or(0);
    Ok(format!(
        "rx {:.1} MiB / tx {:.1} MiB",
        rx as f64 / 1024.0 / 1024.0,
        tx as f64 / 1024.0 / 1024.0
    ))
}

fn orphans() -> AppResult<String> {
    Ok(orphans_value()?)
}

fn orphans_value() -> AppResult<String> {
    if require_cmd("pacman").is_ok() {
        let out = output("pacman", &["-Qtdq"])?;
        let text = String::from_utf8_lossy(&out.stdout);
        let count = text.lines().filter(|l| !l.trim().is_empty()).count();
        return Ok(format!("{count} (pacman)"));
    }
    if require_cmd("deborphan").is_ok() {
        let out = output("deborphan", &[])?;
        let text = String::from_utf8_lossy(&out.stdout);
        let count = text.lines().filter(|l| !l.trim().is_empty()).count();
        return Ok(format!("{count} (deborphan)"));
    }
    if require_cmd("dnf").is_ok() {
        let out = output("dnf", &["repoquery", "--unneeded"])?;
        let text = String::from_utf8_lossy(&out.stdout);
        let count = text.lines().filter(|l| !l.trim().is_empty()).count();
        return Ok(format!("{count} (dnf)"));
    }
    if require_cmd("zypper").is_ok() {
        let out = output("zypper", &["packages", "--orphaned"])?;
        let text = String::from_utf8_lossy(&out.stdout);
        let count = text.lines().filter(|l| l.trim_start().starts_with('i')).count();
        return Ok(format!("{count} (zypper)"));
    }
    Ok("unsupported".to_string())
}

fn processes_value() -> AppResult<String> {
    if require_cmd("ps").is_err() {
        return Ok("ps not installed".to_string());
    }
    let out = output("ps", &["-eo", "pid,comm,%cpu,%mem", "--sort=-%cpu"])?;
    let text = String::from_utf8_lossy(&out.stdout);
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("PID COMMAND %CPU %MEM");
    let mut rows: Vec<String> = lines.take(8).map(|l| l.trim().to_string()).collect();
    rows.sort_by(|a, b| {
        let parse = |s: &String| {
            s.split_whitespace()
                .nth(2)
                .and_then(|v| v.parse::<f32>().ok())
                .unwrap_or(0.0)
        };
        parse(b)
            .partial_cmp(&parse(a))
            .unwrap_or(Ordering::Equal)
    });
    let mut output = String::new();
    output.push_str(header);
    for row in rows {
        output.push_str(" | ");
        output.push_str(&row);
    }
    Ok(output)
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

fn default_interface() -> AppResult<Option<String>> {
    if let Ok(routes) = fs::read_to_string("/proc/net/route") {
        for line in routes.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "00000000" {
                return Ok(Some(parts[0].to_string()));
            }
        }
    }
    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name != "lo" {
                return Ok(Some(name));
            }
        }
    }
    Ok(None)
}

fn read_u64(path: &Path) -> Option<u64> {
    fs::read_to_string(path)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
}
