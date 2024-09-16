use std::str::FromStr;

use social::pb::social::{
    social_service_client::SocialServiceClient, GreetRequest, PostFeedRequest,
};
use tonic::{
    transport::{Channel, Endpoint},
    Request,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("starting social service client");
    let addr = Endpoint::from_str("http://[::1]:9090")?;
    let mut client = SocialServiceClient::connect(addr).await?;

    call_greet(&mut client).await?;
    call_post_feed(&mut client).await?;
    Ok(())
}

async fn call_greet(client: &mut SocialServiceClient<Channel>) -> anyhow::Result<()> {
    let request = Request::new(GreetRequest {
        msg: "John".to_string(),
    });

    println!("call greet with request: {:?}", request);

    let response = client.greet(request).await?;

    let greet_resp = response.into_inner();
    println!("get greet_resp from greet: {:?}", greet_resp);
    Ok(())
}

async fn call_post_feed(client: &mut SocialServiceClient<Channel>) -> anyhow::Result<()> {
    let request = Request::new(PostFeedRequest { limit: 10 });
    println!("call greet with request: {:?}", request);

    let response = client.post_feed(request).await?;
    let mut resp = response.into_inner();
    while let Some(post_resp) = resp.message().await? {
        println!("get posts: {:?}", post_resp.posts);
    }

    Ok(())
}
