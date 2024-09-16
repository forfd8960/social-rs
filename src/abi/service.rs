use std::{ops::Deref, pin::Pin, sync::Arc};

use futures::Stream;
use tonic::{async_trait, Request, Response, Status};

use crate::pb::social::{
    social_service_server::{SocialService, SocialServiceServer},
    GreetRequest, GreetResponse, PostFeedRequest, PostFeedResponse,
};

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<PostFeedResponse, Status>> + Send>>;

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
