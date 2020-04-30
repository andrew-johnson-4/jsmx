#[macro_use] extern crate lazy_static;
extern crate serde_json;
use serde_json::{Value};
use std::collections::HashMap;
use std::rc::Rc;

pub struct SharedExchange;
impl SharedExchange {
   pub fn new() -> SharedExchange {
      SharedExchange {}
   }
   pub fn push(&self, _descriptor_prefix: &str, _descriptor_suffix: &str, _msg: &Value) {
   }
   pub fn listen<F>(&self, _selector_prefix: &str, _selector_suffix: &str, _callback: F)
      where F: 'static + FnMut(&Value) {
   }
}

lazy_static! {
   pub static ref JSMX_EXCHANGE: SharedExchange = SharedExchange::new();
}

pub struct MessageExchange {
   boxes: HashMap<String, Vec<Inbox>>
}
impl MessageExchange {
   pub fn new() -> MessageExchange {
      MessageExchange {
         boxes: HashMap::new()
      }
   }
   pub fn publish(self, descriptor_prefix: &str) -> Outbox {
      Outbox {
         message_exchange: Box::new(self),
         descriptor_prefix: descriptor_prefix.to_string()
      }
   }
   pub fn subscribe(&mut self, selector_prefix: &str, inbox: &Inbox) {
      if !self.boxes.contains_key(selector_prefix) {
         self.boxes.insert(selector_prefix.to_string(), Vec::new());
      }
      self.boxes.get_mut(selector_prefix).unwrap().push((*inbox).clone());
   }
}

pub struct Outbox {
   message_exchange: Box<MessageExchange>,
   descriptor_prefix: String
}
impl Outbox {
   pub fn push(&self, descriptor_suffix: &str, msg: &serde_json::Value) {
       if let Some(inboxes) = self.message_exchange.boxes.get(&self.descriptor_prefix) {
          for inbox in inboxes.iter() {
             if descriptor_suffix == inbox.selector_suffix {
                (inbox.callback)(msg);
             }
          }
       }
   }
}

#[derive(Clone)]
pub struct Inbox {
   selector_suffix: String,
   callback: Rc<Box<dyn Fn(&serde_json::Value)>>
}
impl Inbox {
   pub fn new<F>(selector_suffix: &str, callback: F) -> Inbox
   where F: 'static + Fn(&serde_json::Value)
   {
      Inbox {
         selector_suffix: selector_suffix.to_string(),
         callback: Rc::new(Box::new(callback))
      }
   }
}
