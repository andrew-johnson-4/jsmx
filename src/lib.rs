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
