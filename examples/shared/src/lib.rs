use wasm_bindgen::prelude::*;
use web_sys::console;
use jsmx::{JSMX_EXCHANGE};
#[macro_use] extern crate serde_json;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    JSMX_EXCHANGE.listen("shared", "exchange", |msg| {
       console::log_1(&JsValue::from_str("shared exchange received message:"));
       console::log_1(&JsValue::from_serde(msg).expect("serializable serde json"));
    });

    JSMX_EXCHANGE.push("shared", "exchange", &json!(null));
    JSMX_EXCHANGE.push("shared", "exchange", &json!(["an", "array"]));

    Ok(())
}
