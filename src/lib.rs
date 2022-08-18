#![no_main]

#[macro_use]
extern crate napi_derive;

use napi::{Error, JsNumber, JsString, Result, Status};
use std::ffi::OsString;
use std::process::Command;
use windows::Win32::System::Console::SetConsoleCtrlHandler;

#[napi]
fn ctrlc(pid_from_js: JsNumber, killer_exe_path: JsString) -> Result<bool> {
  let pid = match pid_from_js.get_int32() {
    Ok(child) => child,
    Err(error) => {
      return Err(napi::Error {
        status: Status::GenericFailure,
        reason: format!("unable to parse passed pid: {}", error),
      });
    }
  };
  let killer_exe = match OsString::try_from(killer_exe_path.into_utf8()?.into_owned()?) {
    Ok(child) => child,
    Err(error) => {
      return Err(napi::Error {
        status: Status::GenericFailure,
        reason: format!("unable to kind process killer executable: {}", error),
      });
    }
  };

  unsafe {
    SetConsoleCtrlHandler(None, true);
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

  unsafe { SetConsoleCtrlHandler(None, false) };

  match status {
    Ok(status) if status.success() => Ok(true),
    Ok(_) => Err(Error::new(
      Status::GenericFailure,
      format!("unable to kill process with pid '{:?}'", pid),
    )),
    Err(error) => Err(Error::new(Status::GenericFailure, format!("{}", error))),
  }
}
