use crate::cli::SnapshotsCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: SnapshotsCmd) -> AppResult<()> {
    match cmd {
        SnapshotsCmd::List => run("btrfs", &["subvolume", "list", "/"]),
        SnapshotsCmd::Create { name } => run(
            "btrfs",
            &[
                "subvolume",
                "snapshot",
                "/",
                &format!("/.snapshots/{name}"),
            ],
        ),
        SnapshotsCmd::Delete { name } => run(
            "btrfs",
            &["subvolume", "delete", &format!("/.snapshots/{name}")],
        ),
    }
}
