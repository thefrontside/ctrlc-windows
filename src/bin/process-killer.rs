#[cfg(windows)]
use std::error::Error;

#[cfg(windows)]
use windows::Win32::System::Console::{
  AttachConsole, FreeConsole, GenerateConsoleCtrlEvent, SetConsoleCtrlHandler, CTRL_C_EVENT,
};

#[cfg(windows)]
pub fn main() -> Result<(), Box<dyn Error>> {
  let pid = match std::env::args().nth(1) {
    Some(arg) => arg.parse::<u32>(),
    None => return Err("no pid provided".into()),
  };
  match pid {
    Ok(pid) => kill_pid(pid),
    Err(error) => Err(Box::new(error)),
  }
}

#[cfg(windows)]
fn kill_pid(pid: u32) -> Result<(), Box<dyn Error>> {
  unsafe {
    FreeConsole();

    if AttachConsole(pid).as_bool() {
      SetConsoleCtrlHandler(None, true);
      GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0);
      Ok(())
    } else {
      Err(format!("unable to attach to process {} console", pid).into())
    }
  }
}

#[cfg(unix)]
pub fn main() {
  println!("this program is not designed to work on non-windows platforms");
  std::process::exit(1);
}
