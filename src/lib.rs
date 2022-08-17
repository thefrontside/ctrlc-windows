#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::*;
use std::ffi::OsString;
use std::process::Command;
use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::um::consoleapi::*;

#[napi]
fn ctrlc(pid_from_js: JsNumber, killer_exe_path: JsString) -> Result<bool> {
  let pid = pid_from_js.get_int32().unwrap();
  let killer_exe = OsString::from(killer_exe_path.into_utf8().unwrap().into_owned().unwrap());

  unsafe {
    SetConsoleCtrlHandler(None, TRUE);
  };

  let mut killer = match Command::new(killer_exe).arg(format!("{:?}", pid)).spawn() {
    Ok(child) => child,
    Err(error) => {
      return Err(napi::Error {
        status: Status::GenericFailure,
        reason: format!("unable to spawn process to kill pid: {}", error),
      });
    }
  };

  let status = killer.wait();

  unsafe { SetConsoleCtrlHandler(None, FALSE) };

  match status {
    Ok(status) if status.success() => Ok(true),
    Ok(_) => Err(Error::new(
      Status::GenericFailure,
      format!("unable to kill process with pid '{:?}'", pid),
    )),
    Err(error) => Err(Error::new(Status::GenericFailure, format!("{}", error))),
  }
}
