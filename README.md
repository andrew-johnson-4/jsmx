# jsmx
PubSub Message Passing for Rust with JSON

## Motivation

JSMX is intended for use as a simple message exchange, provided that all messaging is serialized as json objects.
Allowed these restrictions, the message exchange shall obviate the creation, management, and destruction of necessary mutexes and channels.

## Vocabulary

**Message Exchange**: a hub for incoming and outgoing messages

**Shared Exchange**: a thread-safe hub for incoming and outgoing messages

**Message**: a json object


**Inbox**: a subscription to a message exchange feed

**selector prefix**: the first part of the inbox feed address

**selector suffix**: the second part of the inbox feed address

**callback**: a function that will be called when a message is routed to this endpoint


**Outbox**: a feed where related messages can be published

**descriptor prefix**: the first part of the outbox feed route

**descriptor suffix**: the second part of the outbox feed route



## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in jsmx by you,
shall be dual licensed under the MIT and Apache 2.0 license without any additional terms or conditions.
