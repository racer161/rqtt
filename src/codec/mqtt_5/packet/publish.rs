#[derive(Debug, Clone, PartialEq)]
pub struct Publish {
    pub dup: bool,
    pub qos: QoS,
    pub retain: bool,
    pub topic_name: String,
    pub pkid: Option<PacketIdentifier>,
    pub payload: Arc<Vec<u8>>
}