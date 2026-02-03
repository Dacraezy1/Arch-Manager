use crate::cli::HardwareCmd;
use crate::error::AppResult;
use crate::utils::cmd::run;

pub fn handle(cmd: HardwareCmd) -> AppResult<String> {
    match cmd {
        HardwareCmd::Cpu => run("lscpu", &[]),
        HardwareCmd::Mem => run("free", &["-h"]),
        HardwareCmd::Gpu => run("lspci", &["-k"]),
        HardwareCmd::Usb => run("lsusb", &[]),
        HardwareCmd::Pci => run("lspci", &[]),
    }
}
