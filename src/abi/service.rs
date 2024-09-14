use std::{ops::Deref, sync::Arc};

use tonic::{async_trait, Request, Response, Status};

use crate::pb::social::{
    social_service_server::{SocialService, SocialServiceServer},
    GreetRequest, GreetResponse,
};

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
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        println!("get request: {:?}", request);

        let req = request.into_inner();

        let resp = format!("Hello {}, Welcome!", req.msg);
        Ok(Response::new(GreetResponse { msg: resp }))
    }
}

impl Deref for SocialServiceImpl {
    type Target = SocialServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
