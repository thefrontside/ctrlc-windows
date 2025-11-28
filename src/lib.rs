#![no_main]

#[macro_use]
extern crate napi_derive;

use windows::Win32::System::Console::SetConsoleCtrlHandler;

#[napi]
fn ignore_signal(ignore: bool) {
  unsafe {
    SetConsoleCtrlHandler(None, ignore);
  }
}
