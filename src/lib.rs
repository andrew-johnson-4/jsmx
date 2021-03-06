#[macro_use] extern crate lazy_static;
extern crate serde_json;
use serde_json::{Value};
use std::collections::HashMap;
use std::sync::Mutex;
use std::rc::Rc;

pub struct SharedExchange {
   boxes: Mutex<HashMap<String, HashMap<String,Vec<Box<dyn FnMut(&Value) + 'static + Send + Sync>>>>>
}
impl SharedExchange {
   pub fn new() -> SharedExchange {
      SharedExchange {
         boxes: Mutex::new(HashMap::new())
      }
   }
   pub fn push(&self, descriptor_prefix: &str, descriptor_suffix: &str, msg: &Value) {
       if let Some(inbox) = self.boxes.lock().unwrap().get_mut(descriptor_prefix) {
       if let Some(callbacks) = inbox.get_mut(descriptor_suffix) {
          for cb in callbacks.iter_mut() {
             cb(msg);
          }
       }}
   }
   pub fn listen<F>(&self, selector_prefix: &str, selector_suffix: &str, callback: F)
      where F: FnMut(&Value) + 'static + Send + Sync {
       let mut boxes = self.boxes.lock().unwrap();
       if !boxes.contains_key(selector_prefix) {
         boxes.insert(selector_prefix.to_string(), HashMap::new());
       }
       if let Some(inbox) = boxes.get_mut(selector_prefix) {
       if !inbox.contains_key(selector_suffix) {
         inbox.insert(selector_suffix.to_string(), Vec::new());
       }
       if let Some(callbacks) = inbox.get_mut(selector_suffix) {
          callbacks.push(Box::new(callback));
       }}
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
