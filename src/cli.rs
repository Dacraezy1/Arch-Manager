use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "arch-manager")]
#[command(about = "Full-featured Arch Linux system manager in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Pacman(PacmanCmd),
    #[command(subcommand)]
    Aur(AurCmd),
    #[command(subcommand)]
    Systemd(SystemdCmd),
    #[command(subcommand)]
    Users(UsersCmd),
    #[command(subcommand)]
    Network(NetworkCmd),
    #[command(subcommand)]
    Logs(LogsCmd),
    #[command(subcommand)]
    Hardware(HardwareCmd),
    #[command(subcommand)]
    Disks(DisksCmd),
    #[command(subcommand)]
    Snapshots(SnapshotsCmd),
    #[command(subcommand)]
    Health(HealthCmd),
    #[command(subcommand)]
    Updates(UpdatesCmd),
    #[command(subcommand)]
    News(NewsCmd),
    #[command(subcommand)]
    Metrics(MetricsCmd),
}

#[derive(Subcommand)]
pub enum PacmanCmd {
    Update,
    Upgrade,
    Install { pkgs: Vec<String> },
    Remove { pkgs: Vec<String> },
    Search { query: String },
    Info { pkg: String },
    Clean,
}

#[derive(Subcommand)]
pub enum AurCmd {
    Install { pkg: String },
    Update,
}

#[derive(Subcommand)]
pub enum SystemdCmd {
    Status { service: String },
    Start { service: String },
    Stop { service: String },
    Restart { service: String },
    Enable { service: String },
    Disable { service: String },
    List,
}

#[derive(Subcommand)]
pub enum UsersCmd {
    Add { user: String },
    Del { user: String },
    Passwd { user: String },
    List,
}

#[derive(Subcommand)]
pub enum NetworkCmd {
    List,
    Up { iface: String },
    Down { iface: String },
    WifiScan,
}

#[derive(Subcommand)]
pub enum LogsCmd {
    Tail { lines: u32 },
    Service { service: String },
    Boot,
}

#[derive(Subcommand)]
pub enum HardwareCmd {
    Cpu,
    Mem,
    Gpu,
    Usb,
    Pci,
}

#[derive(Subcommand)]
pub enum DisksCmd {
    List,
    Mounts,
}

#[derive(Subcommand)]
pub enum SnapshotsCmd {
    List,
    Create { name: String },
    Delete { name: String },
}

#[derive(Subcommand)]
pub enum HealthCmd {
    Summary,
    Full,
    Services,
    Disk,
    Memory,
    Cpu,
    Kernel,
}

#[derive(Subcommand)]
pub enum MetricsCmd {
    All,
    Ram,
    CpuTemp,
    Gpu,
    Battery,
    Load,
    Uptime,
}

#[derive(Subcommand)]
pub enum UpdatesCmd {
    Check,
    List,
}

#[derive(Subcommand)]
pub enum NewsCmd {
    Latest,
}
