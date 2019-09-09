mod fixed_header;
mod properties;
mod reason_code;
mod low_level_read;
mod payload;
mod qos;


mod connect;

use futures::io::{BufReader};

use payload::Payload;
use low_level_read::*;

//TODO implement this as a trait of a BufferedStream when Rust 
pub async fn read_packet(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>)// -> Packet
{
    //read the fixed header of the control packet
    let header = fixed_header::read_control_packet_type_and_flags(&mut reader).await;

    let remaining_length = low_level_read::read_variable_byte_integer(&mut reader).await;

	println!("header: {:?}", fixed_header::FixedHeader::CONNECT);

}

/*
#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
	CONNECT(VariableHeader, Payload),
	CONNACK(VariableHeader),
	PUBLISH(VariableHeader, Payload),
	PUBACK(VariableHeader),
	PUBREC(VariableHeader),
	PUBREL(VariableHeader),
	PUBCOMP(VariableHeader),
	SUBSCRIBE(VariableHeader, Payload),
	SUBACK(VariableHeader, Payload),
	UNSUBSCRIBE(VariableHeader, Payload),
	UNSUBACK(VariableHeader, Payload),
	PINGREQ,
	PINGRESP,
	DISCONNECT,
	AUTH
}*/
