use futures::prelude::*;
use futures::io::{BufReader};
use std::convert::TryInto;

///adapted from: https://www.reddit.com/r/rust/comments/3xgeo0/biginner_question_how_can_i_get_the_value_of_a/
/// gets the bit at position `n`
/// MQTT: Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant bit, the least significant bit is assigned bit number 0.
pub fn get_bit_at(input: u8, n: u8) -> bool 
{
    if n < 8 { input & (1 << n) != 0 } 
    else { panic!("Tried to read bit outside of range!"); }
}

//TODO: Navigate the dangerous waters of asynchronously reading from an Async
//BufferedReader without copying into a local variable
//I.E. Implement these traits using zero-copy 
//basically we'll have to manage buffer flushing by hand
//but it should give us a noticable decrease in memory usage

//Alternatively it could be better to allocate local variables 
//if we could build different parts of our packet asynchronously
//however we should still be able to implement this without allocating local variables
//it just sounds like a lifetime definition nightmare
//I'm not sure how lifetimes interact with the buffer and according to the github page 
//it may not be implemented for multiple async lifetimes on a single task yet. 
trait MQTTPacketStream
{
    fn read_byte(&mut self) -> std::io::Result<u8>;
    fn read_two_byte_integer(&mut self) -> std::io::Result<u16>;
    fn read_four_byte_integer(&mut self) -> std::io::Result<u32>;
    fn read_utf8_encoded_string(&mut self) -> String;
}

//TODO: Theres currently no support for async trait functions because presumably they need to 
//be wrapped in ARC to be thread safe (but maybe not in all cases)
//at any rate that functionality isn't built into the compiler right now
//but when it is we'll convert this to a trait

//impl MQTTPacketStream for BufReader<runtime::net::tcp::TcpStream>
//{
    pub async fn read_byte(reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> u8
    {
        let mut buffer : [u8;1] = [0];
        reader.read_exact(&mut buffer).await;
        //TODO: convert this to a result to pass it up
        u8::from_be_bytes(buffer.try_into().unwrap())
    }

    pub async fn read_two_byte_integer(reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> u16
    {
        let mut buffer : [u8;2] = [0,0];
        reader.read_exact(&mut buffer).await;
        //TODO: convert this to a result to pass it up
        u16::from_be_bytes(buffer.try_into().unwrap())
    }

    pub async fn read_four_byte_integer(reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> u32
    {
        let mut buffer : [u8;4] = [0,0,0,0];
        reader.read_exact(&mut buffer).await;
        //TODO: convert this to a result to pass it up
        u32::from_be_bytes(buffer.try_into().unwrap())
    }

    pub async fn read_utf8_encoded_string(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> String
    {
        let length : u16 = read_two_byte_integer(&mut reader).await;
        let mut buffer : Vec<u8> = Vec::with_capacity(length as usize);
        reader.read_exact(&mut buffer).await;

        String::from_utf8(buffer).unwrap()
    }
//}