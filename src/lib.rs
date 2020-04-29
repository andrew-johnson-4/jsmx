extern crate serde_json;

/*
   TODO:
   1) define pub sub actor Message Exchange protocols
     a) MessageExchange
        * publish(descriptor_prefix) -> Outbox
        * subscribe(selector, Inbox)
     b) Outbox
        * push(descriptor_suffix, json message)
     c) Inbox
        * new(selector, callback) -> Inbox
   2) define setInterval as a message exchange
   3) derive example to demonstrate clock skew and correction
*/

pub struct MessageExchange;
impl MessageExchange {
   pub fn new() -> MessageExchange {
      MessageExchange {}
   }
   pub fn publish(&self, descriptor_prefix: &str) -> Outbox {
      Outbox {}
   }
   pub fn subscribe(&self, selector_prefix: &str, inbox: &Inbox) {
   }
}

pub struct Outbox;
impl Outbox {
   pub fn push(&self, descriptor_suffix: &str, msg: &serde_json::Value) {
   }
}

pub struct Inbox;
impl Inbox {
   pub fn new<F>(selector_suffix: &str, callback: F) -> Inbox
   where F: Fn(&serde_json::Value)
   {
      Inbox {}
   }
}
