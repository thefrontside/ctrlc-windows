use neon::prelude::*;

#[cfg_attr(target_family="windows", path="windows.rs")]
#[cfg_attr(target_family="unix", path="unix.rs")]
mod ctrlc;
use ctrlc::ctrlc;

register_module!(mut cx, {
    cx.export_function("ctrlc", ctrlc)?;
    Ok(())
});
