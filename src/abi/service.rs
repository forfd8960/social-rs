use std::{ops::Deref, pin::Pin, sync::Arc, time::Duration};

use futures::Stream;
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, Request, Response, Status};

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

pub struct SocialServiceInner {}

impl SocialServiceImpl {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SocialServiceInner {}),
        }
    }

    pub fn into_server(self) -> SocialServiceServer<Self> {
        SocialServiceServer::new(self)
    }
}

// fn init_post_sender() -> Receiver<Post> {
//     let (tx, rx) = tokio::sync::mpsc::channel(CHAN_SIZE);
//     tokio::spawn(async move {
//         loop {
//             println!("send post to sender");
//             let _ = tx.send(Post::fake()).await.map_err(|err| {
//                 warn!("send post failed: {:?}", err);
//             });

//             sleep(Duration::from_secs(5)).await;
//         }
//     });
//     rx
// }

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
    ) -> Result<Response<Self::PostFeedStream>, Status> {
        let req = request.into_inner();
        println!("req: {:?}", req);

        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        tokio::spawn(async move {
            loop {
                println!("send post to sender");

                let response = PostFeedResponse {
                    posts: vec![Post::fake()],
                };
                tx.send(Ok(response)).await.unwrap();

                sleep(Duration::from_secs(3)).await;
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Deref for SocialServiceImpl {
    type Target = SocialServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
