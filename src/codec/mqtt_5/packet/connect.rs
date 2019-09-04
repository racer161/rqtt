#[derive(Debug, Clone, PartialEq)]
pub struct Connect {
	pub protocol: Protocol,
    pub keep_alive: u16,
    pub client_id: String,
	pub clean_session: bool,
    pub last_will: Option<LastWill>,
    pub username: Option<String>,
    pub password: Option<String>
}

//TODO: convert this to a trait implementation when async/await support is availible for traits

//impl Connect
//{
    pub async fn read_connect_packet() -> Connect
    {
        
    }
//}