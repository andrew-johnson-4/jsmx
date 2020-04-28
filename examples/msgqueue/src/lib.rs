use wasm_bindgen::prelude::*;
use web_sys::console;

extern crate jsmx;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
