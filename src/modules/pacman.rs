use crate::cli::PacmanCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: PacmanCmd) -> AppResult<()> {
    match cmd {
        PacmanCmd::Update => run("pacman", &["-Sy"]),
        PacmanCmd::Upgrade => run("pacman", &["-Syu"]),
        PacmanCmd::Install { pkgs } => {
            let mut args = vec!["-S"];
            for p in &pkgs {
                args.push(p);
            }
            run("pacman", &args)
        }
        PacmanCmd::Remove { pkgs } => {
            let mut args = vec!["-Rns"];
            for p in &pkgs {
                args.push(p);
            }
            run("pacman", &args)
        }
        PacmanCmd::Search { query } => run("pacman", &["-Ss", &query]),
        PacmanCmd::Info { pkg } => run("pacman", &["-Si", &pkg]),
        PacmanCmd::Clean => run("pacman", &["-Sc"]),
    }
}
