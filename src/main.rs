use std::net::SocketAddr;

use social::abi::service::SocialServiceImpl;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("starting social service server");

    let addr: SocketAddr = "[::1]:9090".parse().unwrap();

    println!("init social service server...");
    let service = SocialServiceImpl::new().into_server();

    println!("serve social service at {}", addr);
    Server::builder().add_service(service).serve(addr).await?;
    Ok(())
}
