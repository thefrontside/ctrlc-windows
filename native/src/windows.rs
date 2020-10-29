use neon::prelude::*;
use winapi::shared::minwindef::{TRUE, FALSE};

use winapi::um::consoleapi::*;
use winapi::um::wincon::*;
use std::process::Command;

// https://gist.github.com/rdp/f51fb274d69c5c31b6be

pub fn ctrlc(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let pid = cx.argument::<JsNumber>(0)?.value() as u32;
    unsafe {
        // Once you call FreeConsole, there's no way to get
        // back to it. This starts a placeholder process that inherits
        // our console so that we can reattach to it later.
        let mut holder = match Command::new("cmd.exe").spawn() {
            Ok(child) => child,
            Err(_) => return cx.throw_error("unable to retain console")
        };

        // release our current console
        FreeConsole();

        // attach to the console of the process we want to send ctrl-c to
        if AttachConsole(pid) != 0 {
            SetConsoleCtrlHandler(None, TRUE);
            GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0);
            FreeConsole();
            AttachConsole(holder.id());

            SetConsoleCtrlHandler(None, FALSE);

            match holder.kill() {
                Ok(_) => Ok(cx.undefined()),
                Err(_) => cx.throw_error("unable to kill off placeholder process")
            }

        } else {
            match holder.kill() {
                Ok(_) => Ok(cx.undefined()),
                Err(_) => cx.throw_error("unable to kill off placeholder process")
            }?;
            cx.throw_error(format!("unable to attach to console of  {pid}", pid=pid))
        }
    }
}
