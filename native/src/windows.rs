use neon::prelude::*;
use winapi::shared::minwindef::{TRUE, FALSE};
use winapi::um::consoleapi::*;
use std::process::{Command};

pub fn ctrlc(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let pid = cx.argument::<JsNumber>(0)?.value() as u32;
    let killer_exe = cx.argument::<JsString>(1)?.value() as String;

    unsafe {
        SetConsoleCtrlHandler(None, TRUE);
    };

    let mut killer = match Command::new(killer_exe)
        .arg(format!("{}", pid))
        .spawn() {
            Ok(child) => child,
            Err(error) => return cx.throw_error(format!("unable to retain console: {}", error))
        };

    let status = killer.wait();

    unsafe { SetConsoleCtrlHandler(None, FALSE) };

    match status {
        Ok(status) if status.success() => {
            Ok(cx.undefined())
        },
        Ok(_) => cx.throw_error("killed with bad exit"),
        Err(_) => cx.throw_error("bad")
    }
}
