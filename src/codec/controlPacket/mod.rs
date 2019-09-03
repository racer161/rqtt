mod publish;
mod control_packet_fixed_header;
mod low_level_read;

use futures::io::{BufReader};

struct MQTTControlPacket
{
    fixed_header : control_packet_fixed_header::ControlPacketFixedHeader,
    //variable_header : Some()
    payload : Vec<u8>
}


//TODO implement this as a trait of a BufferedStream when Rust 
async fn read_packet(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> MQTTControlPacket
{
    let fixed_header = control_packet_fixed_header::read_control_packet_type_and_flags(&mut reader).await;

    

    MQTTControlPacket
    {
        fixed_header : fixed_header,
        payload : Vec::new()
    }
}