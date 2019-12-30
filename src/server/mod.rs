use async_std::net::{TcpListener};

use async_std::io::{BufReader};
use async_std::stream::{ StreamExt};
use async_std::task;

use mqtt_5::data_representation::FromBitReader;
use bitreader_async::BitReader;

use std::time::{Duration, Instant};

pub async fn new(addr_string : String) -> async_std::io::Result<()>
{
    let listener = TcpListener::bind(addr_string).await?;
    println!("Listening on {}", listener.local_addr()?);

    // accept connections and process them
    //TODO: change this back to try_for_each_concurrent when its availible
    //OR look into futures-rs which still has it and may be compatible
    let _result = listener.incoming().try_for_each(|tcp_stream| 
    {
        let stream = tcp_stream?;

        let _handle = task::spawn(async move {

            //the handler itself is also async
            println!("Accepting from: {:#?}", stream.peer_addr());

            //let (reader, writer) = &mut stream.split();

            let b_reader : BufReader<async_std::net::TcpStream> = BufReader::new(stream);

            let mut bitreader = BitReader::new(b_reader);

            let now = Instant::now();

            loop {

                let packet = mqtt_5::Packet::from_bitreader(&mut bitreader).await;

                match packet
                {
                    Ok(p) => {println!("milis : {}", now.elapsed().as_millis()); print!("Packet : {:#?}", p)},
                    Err(e) => panic!(e)
                }

                
                //handler::handle_packet(& packet);
            }

            //let mut b_writer : BufWriter<async_std::net::TcpStream> = BufWriter::new(stream);
        });
            
        Ok::<(), std::io::Error>(())
            
    }).await?;

    Ok(())
}