use wasm_bindgen::prelude::*;
use web_sys::console;

extern crate jsmx;
use jsmx::{MessageExchange, Outbox, Inbox};
use serde_json::{Value};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let exchange = MessageExchange::new();

    let inbox = Inbox::new("1000", move |msg| {
        console::log_1(&JsValue::from_str("Hello mail inbox!"));
    });
    exchange.subscribe("setInterval", &inbox);

    let outbox = exchange.publish("setInterval");
    outbox.push("1000", &Value::Null);

    Ok(())
}
