# RQTT
A Rust based MQTT server with focuses on
- Asynchronous scaling using the new async/await syntax in Rust 
- Zero-copy parsing using asynchronous buffered reads

## Motivation
---

The world's need for low latency high throughput connections for practical devices is growing everyday. MQTT started out as a protocol for industrial machinery to communicate with a host of managing and regulating servers but quickly progressed into not only what most IoT devices use but also mainstream messaging applicaitons. The need is clear for a robust version of this popular protocol and what better language to write it in than the one that claims to be the language for software that will stand the test of time. This project aims to create a safe, scalable and robust server for these small but important communications in our everyday lives. 

This project also aims to include as few 3rd party libraries as possible and to be no-std compatible since many instances of rqtt will be running on devices without an OS.

## RQTT in the Pipeline Eco-system
MQTT is the protocol layer used in the [Pipeline] distributed version control platform. RQTT was created as a robust and scalable message server for the "pipe" version control protocol which is communicated over MQTT messages. Rust was targeted as the language for rqtt not only because of its safety gaurantees but because the rest of the Pipeline version control specification is implemented in Rust. This allows Pipeline to be compiled as one program with a single runtime managing all asynchronous threads. It also allows the pieces necessary for the in browser version of Pipeline to be compiled to one small webassembly package including the rqtt client. 

## Development Approach
rqtt tries to follow the MQTT guidelines to the letter where possible and follows Rust code 
best practices where the MQTT 5.0 specification is ambiguous. The guidlines for MQTT version 5.0 are specified [here.](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901000)

## Timeline
#### For a more detailed list of what is and isn't implemented see this [issue](https://github.com/racer161/rqtt/issues/1#issue-490671750)
Progress:
- [x] 1.0 Data Representation - 9/1/2019
- [x] 2.0 Control Packet Format Parsing - 9/16/2019
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

## RQTT and rumqttd
rqtt is not expected to replace [rumqttd](https://github.com/tekjar/rumqttd) anytime soon. rqtt is a sibling of rumqttd looking to employ async/await and other optimization techniques that may eventually make their way over to rumqttd. For now, if you're looking for a stable MQTT server and don't have time to wait for async/await to stabilize then you should look to [rumqttd](https://github.com/tekjar/rumqttd).

## Benchmarks and Goals
Since rqtt is a sibling to rumqttd with focuses on scalibility via greater asynchronous integration it makes sense that rqtt would try to perform better than rumqttd in benchmarks. Success in this area would be achieving a lower serve time in this benchmark also used by rumqttd:

#### Subscribe to a wildcard topic & publish 1 million qos1 messages (and wait for acks) to the broker on local machine
 

| Server                  | PUBLISH Packet Processing Time | Average Memory Usage (Bytes)| Total Time Taken For Incoming PUBLISH Packets |
| ------------------------|-------------------------------|------------------------| ---------------------------------------------|
| mosquitto (1.4.11)      | 3m41.531635728s                | 2,417,095,749         | 3m42.242271385s                              |
| rumqttd                 | 3m35.521259143s                | 2,318,325,979         | 3m36.247010478s                              |
| emqttd (2.1.2)          | 5m21.976169243s                | 2,469,802,635         | 5m27.802035293s                              |
| rqtt                    | X                              | X                     | X |
