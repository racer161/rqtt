#[derive(Debug, Clone, PartialEq)]
pub struct Suback {
    pub pkid: PacketIdentifier,
	// (error, qos)
	// TODO: replace with enum
	pub return_codes: Vec<SubscribeReturnCodes>
}