use neon::prelude::*;
use std::process::{Command};
use winapi::um::{wincon, consoleapi};

// https://gist.github.com/rdp/f51fb274d69c5c31b6be

fn free_console() -> Result<(), String> {
    let result = unsafe { wincon::FreeConsole() };
    match result {
        0 => Ok(()),
        _ => Err(format!("unable to free console"))
    }
}

fn attach_console(pid: u32) -> Result<(), String> {
    let result = unsafe { wincon::AttachConsole(pid) };
    match result {
        0 => Ok(()),
        _ => Err(format!("unable to attach to console of {pid}", pid=pid))
    }
}

fn clear_console_ctrl_handler(add: bool) -> Result<(), String> {
    let result = unsafe { consoleapi::SetConsoleCtrlHandler(None, add.into()) };
    match result {
        0 => Ok(()),
        _ => Err(format!("unable to clear ctrls handler"))
    }
}

fn generate_ctrlc_event() -> Result<(), String> {
    let result = unsafe { wincon::GenerateConsoleCtrlEvent(wincon::CTRL_C_EVENT, 0) };
    match result {
        0 => Ok(()),
        _ => Err(format!("unable to generate ctrls event"))
    }
}
fn ctrlc_perform(pid: u32, holder_pid: u32) -> Result<(), String> {
    free_console()?; // free ourselves from our own console
    attach_console(pid)?; // attach to the console of the process we want to send ctrl-c to
    clear_console_ctrl_handler(true)?; // remove our own ctrl-c handler so we don't kill ourselves
    generate_ctrlc_event()?; // generate the ctrlc event in our own console, which is now the console of the target process
    free_console()?; // free ourselves from out own console again
    attach_console(holder_pid)?; // attach to the console of the holder process
    clear_console_ctrl_handler(false)?; // what does this do?!

    Ok(())
}

fn ctrlc_inner(pid: u32) -> Result<(), String> {
    // Once you call FreeConsole, there's no way to get
    // back to it. This starts a placeholder process that inherits
    // our console so that we can reattach to it later.
    let mut holder = Command::new("cmd.exe").spawn().map_err(|_| format!("unable to retain console"))?;

    let result = ctrlc_perform(pid, holder.id());

    holder.kill().map_err(|_| format!("unable to terminate holder process"))?;

    return result;
}

pub fn ctrlc(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let pid = cx.argument::<JsNumber>(0)?.value() as u32;
    match ctrlc_inner(pid) {
        Ok(_) => Ok(cx.undefined()),
        Err(error) => cx.throw_error(error)
    }
}
