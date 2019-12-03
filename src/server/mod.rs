use async_std::net::{TcpListener, TcpStream};

use async_std::io::{BufReader, BufWriter};
use async_std::stream::{self, StreamExt};
use async_std::task;

use std::str;

use mqtt_5::data_representation::FromBitReader;

pub async fn new(addr_string : String) -> async_std::io::Result<()>
{
    let mut listener = TcpListener::bind(addr_string).await?;
    println!("Listening on {}", listener.local_addr()?);

    // accept connections and process them
    //TODO: change this back to try_for_each_concurrent when its availible
    //OR look into futures-rs which still has it and may be compatible
    let result = listener.incoming().try_for_each(|tcp_stream| 
    {
        
        let stream = tcp_stream?;

        let handle = task::spawn(async move {

            //the handler itself is also async
            println!("Accepting from: {:#?}", stream.peer_addr());

            //let (reader, writer) = &mut stream.split();

            //let mut b_reader : BufReader<async_std::net::TcpStream> = BufReader::new(stream);
            //let mut b_writer : BufWriter<async_std::net::TcpStream> = BufWriter::new(stream);
            
            //let packet = mqtt_5::Packet::from_bitreader(reader);
                //print!("{}", str::from_utf8(&byteBuffer).unwrap());
        });
            
        Ok::<(), std::io::Error>(())
            
    }).await?;

    Ok(())
}