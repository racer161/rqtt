use futures::io::{BufReader};
use super::low_level_read::*;

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;


//This file implements the MQTT control packet format as written in the 
//MQTT 5.0 spec here: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901019

pub struct ControlPacketFixedHeader
{
    packet_type : ControlPacketType,
    flags : u8
}

//These values are implicitly valued from 0 to 15 for parsing 
#[derive(FromPrimitive)]
enum ControlPacketType
{
    RESERVED = 0,
    CONNECT,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH
}

impl ControlPacketType
{
    fn from_u8(pType : u8) -> ControlPacketType
    {
        match FromPrimitive::from_u8(pType)
        {
            Some(typ) => return typ,
            None => panic!("couldn't parse control packet type because it was malformed")
        }
    }
}


//pub async fn read_control_packet_fixed_header() -> ControlPacketFixedHeader
//{

//}

pub async fn read_control_packet_type_and_flags(mut reader: &mut BufReader<runtime::net::tcp::TcpStream>) -> ControlPacketFixedHeader
{
    let first_byte = read_byte(&mut reader).await;

    //get the four bit values via bitmask AND
    let raw_type : u8 = (first_byte & 0b11110000u8);
    let raw_flag : u8 = (first_byte & 0b00001111u8);

    //read the control packet type
    let parsedType : ControlPacketType = ControlPacketType::from_u8(raw_type);

    //verify our packet has the correct flags or it will be considered malformed
    let parsedFlag : ControlPacketFlag = ControlPacketFlag::from_u8(raw_flag);

    match parsedType 
    {
        //If its a publish packet we can't really know if its malformed based on the flag
        ControlPacketType::PUBLISH => {},
        ControlPacketType::PUBREL | ControlPacketType::SUBSCRIBE | ControlPacketType::UNSUBSCRIBE => 
        { 
            //These packets should have bit 1 enabled. If they don't they are malformed.
            if parsedFlag != ControlPacketFlag::BIT1 { panic!("Couldn't parse control packet: Packet flags did not match packet header!"); } 
        },
        _ => 
        { 
            //All other packets should have blank flags. If they don't they are malformed.
            if parsedFlag != ControlPacketFlag::BLANK { panic!("Couldn't parse control packet: Packet flags did not match packet header!"); } 
        }
    }

    //return the raw flag in case more processing is needed up stream (eg. QOS handling)
    ControlPacketFixedHeader 
    {
        packet_type : parsedType,
        flags: raw_flag
    }
}


// An enum of bitmask values conctained in u8s
#[derive(PartialEq, PartialOrd)]
enum ControlPacketFlag
{
    //EVERY PACKET EXCEPT PUBLISH, or the exceptions below should return blank flags 
    BLANK,
    //PUBREL, SUBSCRIBE, and UNSUBSCRIBE should return this or 
    //they will be considered malformed
    BIT1,
    PUBLISH(u8)
}

impl ControlPacketFlag
{
    pub fn from_u8(pType : u8) -> ControlPacketFlag
    {
        match pType
        {
            0b0000u8 => ControlPacketFlag::BLANK,
            0b0010u8 => ControlPacketFlag::BIT1,
            _ => panic!("Couldn't parse control packet flag because it was malformed")
        }
    }
}