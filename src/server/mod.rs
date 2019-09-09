use futures::prelude::*;
use runtime::net::TcpListener;
use futures::io::{BufReader};
use std::str;

use super::mqtt_5;

pub struct MQTTServer
{

}

impl MQTTServer
{
    pub async fn new(addr_string : String) -> std::io::Result<TcpListener>
    {
        let mut listener = TcpListener::bind(addr_string)?;
        println!("Listening on {}", listener.local_addr()?);

        // accept connections and process them in parallel
        listener.incoming().try_for_each_concurrent(None, async move |stream| {
        //the setup for the handler is async


                runtime::spawn(async move {
                    //the handler itself is also async

                    println!("Accepting from: {}", stream.peer_addr()?);

                    //let (reader, writer) = &mut stream.split();

                    let mut b_reader : BufReader<runtime::net::tcp::TcpStream> = BufReader::new(stream); 

                    mqtt_5::read_packet(&mut b_reader).await;
                        //print!("{}", str::from_utf8(&byteBuffer).unwrap());
                    
                    Ok::<(), std::io::Error>(())
                })
                .await
            })
            .await?;
        Ok(listener)
    }

}