#[cfg(target_family = "windows")]
use std::error::Error;

#[cfg(target_family = "windows")]
use winapi::um::wincon::*;
#[cfg(target_family = "windows")]
use winapi::um::consoleapi::*;
#[cfg(target_family = "windows")]
use winapi::shared::minwindef::{TRUE};

#[cfg(target_family = "windows")]
pub fn main() -> Result<(), Box<dyn Error>> {
    let pid = match std::env::args().nth(1) {
        Some(arg) => arg.parse::<u32>(),
        None => return Err("no pid provided".into())
    };
    match pid {
        Ok(pid) => kill_pid(pid),
        Err(error) => Err(Box::new(error))
    }
}

#[cfg(target_family = "windows")]
fn kill_pid(pid: u32) -> Result<(), Box<dyn Error>> {
    unsafe {
        FreeConsole();

        if AttachConsole(pid) != 0 {
            SetConsoleCtrlHandler(None, TRUE);
            GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0);
            Ok(())
        } else {
            Err(format!("unable to attach to process {} console", pid).into())
        }
    }
}

#[cfg(target_family = "unix")]
pub fn main() {
    println!("this program is not designed to work on non-windows platforms");
    std::process::exit(1);
}
