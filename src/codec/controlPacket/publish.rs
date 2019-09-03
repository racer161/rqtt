//read the QOS data stored in 
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

const DUP_BIT : u8 = 3;
const RETAIN_BIT : u8 = 0;

//traits for reading the publish flags
pub trait PublishFlags
{
    fn read_qoS(self) -> PublishQoS;
    fn is_duplicate(self) -> bool;
    fn is_retain(self) -> bool;
}

impl PublishFlags for u8
{
    fn read_qoS(self) -> PublishQoS
    {
        let raw_qos = self & 0b0110u8; 

        match FromPrimitive::from_u8(raw_qos)
        {
            Some(typ) => return typ,
            None => panic!("Couldn't parse control packet flag because it was malformed")
        }
    } 

    fn is_duplicate(self) -> bool { get_bit_at(self, DUP_BIT) }

    fn is_retain(self) -> bool { get_bit_at(self, RETAIN_BIT) }

}

#[derive(FromPrimitive)]
pub enum PublishQoS
{
    AtMostOnce = 0,
    AtLeastOnce,
    ExactlyOnce
}

///adapted from: https://www.reddit.com/r/rust/comments/3xgeo0/biginner_question_how_can_i_get_the_value_of_a/
/// gets the bit at position `n`
/// MQTT: Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant bit, the least significant bit is assigned bit number 0.
pub fn get_bit_at(input: u8, n: u8) -> bool 
{
    if n < 8 { input & (1 << n) != 0 } 
    else { panic!("Tried to read bit outside of range!"); }
}