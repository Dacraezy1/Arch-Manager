mod cli;
mod error;
mod modules;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use error::AppResult;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> AppResult<()> {
    match cli.command {
        Commands::Pacman(cmd) => modules::pacman::handle(cmd),
        Commands::Aur(cmd) => modules::aur::handle(cmd),
        Commands::Systemd(cmd) => modules::systemd::handle(cmd),
        Commands::Users(cmd) => modules::users::handle(cmd),
        Commands::Network(cmd) => modules::network::handle(cmd),
        Commands::Logs(cmd) => modules::logs::handle(cmd),
        Commands::Hardware(cmd) => modules::hardware::handle(cmd),
        Commands::Disks(cmd) => modules::disks::handle(cmd),
        Commands::Snapshots(cmd) => modules::snapshots::handle(cmd),
        Commands::Health(cmd) => modules::health::handle(cmd),
        Commands::News(cmd) => modules::news::handle(cmd),
        Commands::Updates(cmd) => modules::updates::handle(cmd),
        Commands::Metrics(cmd) => modules::metrics::handle(cmd),
    }
}
