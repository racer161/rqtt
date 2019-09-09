pub enum Protocol
{
    MQTT(u8),
    MQIsdp
}

impl Protocol
{
    pub fn from_string(protocol : &str, version_level : u8) -> Protocol
    {
        match protocol 
        {
            "MQTT" => Protocol::MQTT(version_level),
            _ => panic!("Couldn't parse control packet because the client tried to use a protocol other than MQTT!")
        }
    }
}