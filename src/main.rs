use social::abi::service::SocialServiceImpl;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    info!("starting social service server");

    let addr: SocketAddr = "[::1]:9090".parse().unwrap();

    info!("init social service server...");
    let service = SocialServiceImpl::new().into_server();

    info!("serve social service at {}", addr);
    Server::builder().add_service(service).serve(addr).await?;
    Ok(())
}
