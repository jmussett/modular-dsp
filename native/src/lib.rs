#[macro_use]
extern crate neon;
extern crate portaudio;
extern crate portmidi;

pub mod audio;
pub mod midi;
pub mod modules;
pub mod portaudiobindings;
pub mod portmidibindings;
pub mod neonbindings;

use neon::prelude::*;

fn play(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello world!"))
}

register_module!(mut m, {
    m.export_function("play", play)
});