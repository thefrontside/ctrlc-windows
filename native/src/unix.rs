use neon::prelude::*;

pub fn ctrlc(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cx.throw_type_error("tried to invoke windows-specific ctrlc on a non-windows platform.")
}
