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

struct MessageExchange;
impl MessageExchange {
   pub fn publish(descriptor_prefix: &str) -> Outbox {
      Outbox {}
   }
   pub fn subscribe(selector_prefix: &str, inbox: &Inbox) {
   }
}

struct Outbox;
impl Outbox {
   pub fn push(descriptor_suffix: &str, msg: &serde_json::Value) {
   }
}

struct Inbox;
/* pub impl Inbox {
   pub fn new(selector_suffix: &str, callback: ?) -> Inbox {
   }
} */
