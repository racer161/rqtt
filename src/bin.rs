use async_std::task::block_on;

//TODO: update to the stable std::async

#[async_std::main]
async fn main() -> std::io::Result<()>
{
    //spawn a new async server
    let server = rqtt::server::new("127.0.0.1:8080".to_owned());
    block_on(server)
}