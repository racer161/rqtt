#[derive(Debug, Clone, PartialEq)]
pub struct Subscribe {
    pub pkid: PacketIdentifier,
	// (topic path, qos)
	pub topics: Vec<SubscribeTopic>
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubscribeTopic {
	pub topic_path: String,
	pub qos: QoS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscribeReturnCodes {
	Success(QoS),
	Failure
}