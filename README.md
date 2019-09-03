# RQTT
---
A Rust based MQTT server with focuses on
- Asynchronous scaling using the new async/await syntax in Rust 
- Zero-copy parsing using asynchronous buffered reads

## Development Approach
RQTT tries to follow the MQTT guidelines to the letter where possible and follows Rust code 
best practices where the MQTT 5.0 specification is ambiguous.

## Timeline
---
Progress:
- [x] 1.0 Data Representation - 9/1/2019
- [ ] 2.0 Control Packet Format Parsing - 9/16/2019
* Reason Codes 
* Payload Specifications and Properties
- [ ] 3.0 MQTT Control Packet Handling - 10/1/2019
* Core routing functionality
* Publishing / Recieving 
* Subscribing/Unsubscribing from a channel
* Ping / Responce handling
- [ ] 4.0 Operational Behavior - 10/15/2019
* Session state management
* Saving undelivered messages 
* Quality of service level
* Topic Names/ Topic Filters
- [ ] 5.0 Security - 10/29/2019
* Authorization via 3rd party firms
- [ ] 6.0 Ensuring Conformance with websockets -11/12/2019
* Add a testing utility in github for each browser
- [ ] 7.0 Beuaracracy and other conformance - TBD - we'll see how much of this is actually coding

## Motivation
---

The world's need for low latency high throughput connections for practical devices is growing everyday. MQTT started out as a protocol for 
