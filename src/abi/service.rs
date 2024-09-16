use std::{ops::Deref, pin::Pin, sync::Arc, time::Duration};

use futures::Stream;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};
use tonic::{async_trait, Request, Response, Status};
use tracing::warn;

use crate::pb::social::{
    social_service_server::{SocialService, SocialServiceServer},
    GreetRequest, GreetResponse, Post, PostFeedRequest, PostFeedResponse,
};

const CHAN_SIZE: usize = 10;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<PostFeedResponse, Status>> + Send>>;

#[derive(Clone)]
pub struct SocialServiceImpl {
    inner: Arc<SocialServiceInner>,
}

pub struct SocialServiceInner {
    post_chan: Receiver<Post>,
}

impl SocialServiceImpl {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SocialServiceInner {
                post_chan: init_post_sender(),
            }),
        }
    }

    pub fn into_server(self) -> SocialServiceServer<Self> {
        SocialServiceServer::new(self)
    }
}

fn init_post_sender() -> Receiver<Post> {
    let (tx, rx) = tokio::sync::mpsc::channel(CHAN_SIZE);
    tokio::spawn(async move {
        loop {
            println!("send post to sender");
            let _ = tx.send(Post::fake()).await.map_err(|err| {
                warn!("send post failed: {:?}", err);
            });

            sleep(Duration::from_secs(5)).await;
        }
    });
    rx
}

#[async_trait]
impl SocialService for SocialServiceImpl {
    async fn greet(&self, request: Request<GreetRequest>) -> ServiceResult<GreetResponse> {
        println!("get request: {:?}", request);

        let req = request.into_inner();

        let resp = format!("Hello {}, Welcome!", req.msg);
        Ok(Response::new(GreetResponse { msg: resp }))
    }

    type PostFeedStream = ResponseStream;
    async fn post_feed(
        &self,
        request: Request<PostFeedRequest>,
    ) -> ServiceResult<Self::PostFeedStream> {
        println!("get request: {:?}", request);
        todo!()
    }
}

impl Deref for SocialServiceImpl {
    type Target = SocialServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
