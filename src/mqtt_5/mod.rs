mod fixed_header;
mod variable_header;
mod low_level_read;
mod payload;

use futures::io::{BufReader};

use variable_header::VariableHeader;
use payload::Payload;
use low_level_read::*;

//TODO implement this as a trait of a BufferedStream when Rust 
async fn read_packet(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>)// -> Packet
{
    //read the fixed header of the control packet
    let fixed_header = fixed_header::read_control_packet_type_and_flags(&mut reader).await;

    let remaining_length = low_level_read::read_variable_byte_integer(&mut reader).await;

	

}


#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
	Connect(VariableHeader, Payload),
	Connack(VariableHeader, Payload),
	Publish(VariableHeader, Payload),
	Puback(VariableHeader, Payload),
	Pubrec(VariableHeader, Payload),
	Pubrel(VariableHeader, Payload),
	Pubcomp(VariableHeader, Payload),
	Subscribe(VariableHeader, Payload),
	Suback(VariableHeader, Payload),
	Unsubscribe(VariableHeader, Payload),
	Unsuback(VariableHeader, Payload),
	Pingreq,
	Pingresp,
	Disconnect
}
