//! A simple echo server.
//!
//! Run the server and connect to it with `nc 127.0.0.1 8080`.
//! The server will wait for you to enter lines of text and then echo them back.
#![feature(async_await)]

use futures::prelude::*;
use runtime::net::TcpListener;
use futures::executor::block_on;


#[runtime::main]
async fn main()
{
    //spawn a new async server
    let server = rqtt::server::MQTTServer::new("127.0.0.1:8080".to_owned());
    block_on(server);
}