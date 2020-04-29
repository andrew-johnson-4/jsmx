extern crate serde_json;
use std::collections::HashMap;

pub struct MessageExchange {
   boxes: HashMap<String, Vec<Inbox>>
}
impl MessageExchange {
   pub fn new() -> MessageExchange {
      MessageExchange {
         boxes: HashMap::new()
      }
   }
   pub fn publish(&self, descriptor_prefix: &str) -> Outbox {
      Outbox {}
   }
   pub fn subscribe(&mut self, selector_prefix: &str, inbox: &Inbox) {
      if !self.boxes.contains_key(selector_prefix) {
         self.boxes.insert(selector_prefix.to_string(), Vec::new());
      }
   }
}

pub struct Outbox;
impl Outbox {
   pub fn push(&self, descriptor_suffix: &str, msg: &serde_json::Value) {
   }
}

pub struct Inbox {
   selector_suffix: String,
   callback: Box<dyn Fn(&serde_json::Value)>
}
impl Inbox {
   pub fn new<F>(selector_suffix: &str, callback: F) -> Inbox
   where F: 'static + Fn(&serde_json::Value)
   {
      Inbox {
         selector_suffix: selector_suffix.to_string(),
         callback: Box::new(callback)
      }
   }
}
