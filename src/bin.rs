use futures::executor::block_on;

#[runtime::main]
async fn main()
{
    //spawn a new async server
    let server = rqtt::server::MQTTServer::new("127.0.0.1:8080".to_owned());
    block_on(server);
}