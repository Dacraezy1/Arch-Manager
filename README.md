# Arch Manager

A full-featured Arch Linux system manager written in Rust. It wraps system tools like `pacman`, `systemctl`, `journalctl`, `nmcli`, `lsblk`, `btrfs`, and more. Users should run it with `sudo` when required.

## Usage

```bash
sudo arch-manager pacman update
sudo arch-manager systemd status sshd
sudo arch-manager health summary
sudo arch-manager health full
sudo arch-manager news latest
```

## Notes
- This tool does not perform privilege escalation. Users must run it with `sudo` for privileged actions.
- Health checks are best-effort: it reports what it can detect based on available tools.

## GUI (Qt)

A simple Qt Widgets GUI is included under `gui/`. It shells out to the Rust CLI.

```bash
cd gui
cmake -S . -B build
cmake --build build
./build/arch-manager-gui
```
