mod publish_flags;
mod fixed_header;
mod low_level_read;
mod packet;

use futures::io::{BufReader};

/*//TODO implement this as a trait of a BufferedStream when Rust 
async fn read_packet(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> Packet
{
    //read the fixed header of the control packet
    let fixed_header = fixed_header::read_control_packet_type_and_flags(&mut reader).await;

    let remaining_length = low_level_read::read_variable_byte_integer(&mut reader).await;



    Packet
    {
        fixed_header : fixed_header,
        payload : Vec::new()
    }
}*/

/*
#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
	Connect(Connect),
	Connack(Connack),
	Publish(Publish),
	Puback(PacketIdentifier),
	Pubrec(PacketIdentifier),
	Pubrel(PacketIdentifier),
	Pubcomp(PacketIdentifier),
	Subscribe(Subscribe),
	Suback(Suback),
	Unsubscribe(Unsubscribe),
	Unsuback(PacketIdentifier),
	Pingreq,
	Pingresp,
	Disconnect
}*/















