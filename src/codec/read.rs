use std::io::{BufReader, Read, Take, Cursor};
use std::net::TcpStream;
use std::sync::Arc;
use byteorder::{ReadBytesExt, BigEndian};

pub trait MQTTRead
{
    
}