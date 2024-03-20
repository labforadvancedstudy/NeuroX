// extensions/messenger_ext.rs
use crate::proto::messenger_ext_server::{
    MessengerExt as MessengerExtTrait, MessengerExtServer, MessengerInExt as MessengerInExtTrait,
    MessengerInExtServer, MessengerOutExt as MessengerOutExtTrait, MessengerOutExtServer,
};
use crate::proto::{
    MessengerExtRequest, MessengerExtResponse, MessengerInExtRequest, MessengerInExtResponse,
    MessengerOutExtRequest, MessengerOutExtResponse,
};
use std::convert::Infallible;
use thiserror::Error;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Error, Debug)]
pub enum MessengerExtError {
    #[error("Channel send error: {0}")]
    ChannelSend(#[from] mpsc::error::SendError<(String, String)>),
    #[error("Messenger API error: {0}")]
    MessengerApi(String),
}

pub struct MessengerInExt {
    sender: mpsc::Sender<(String, String)>,
}

impl MessengerInExt {
    pub fn new(sender: mpsc::Sender<(String, String)>) -> Self {
        Self { sender }
    }
}

#[tonic::async_trait]
impl MessengerInExtTrait for MessengerInExt {
    async fn process_message(
        &self,
        request: Request<MessengerInExtRequest>,
    ) -> Result<Response<MessengerInExtResponse>, Status> {
        let MessengerInExtRequest {
            message_hash_id,
            text,
        } = request.into_inner();
        self.sender.send((message_hash_id, text)).await?;
        Ok(Response::new(MessengerInExtResponse {}))
    }
}

pub struct MessengerOutExt {
    api_client: MessengerApiClient,
}

impl MessengerOutExt {
    pub fn new(api_client: MessengerApiClient) -> Self {
        Self { api_client }
    }
}

#[tonic::async_trait]
impl MessengerOutExtTrait for MessengerOutExt {
    async fn send_message(
        &self,
        request: Request<MessengerOutExtRequest>,
    ) -> Result<Response<MessengerOutExtResponse>, Status> {
        let MessengerOutExtRequest {
            message_hash_id,
            text,
        } = request.into_inner();
        self.api_client.send_message(message_hash_id, text).await?;
        Ok(Response::new(MessengerOutExtResponse {}))
    }
}

struct MessengerApiClient {
    // Placeholder implementation. Replace with actual Messenger API client.
}

impl MessengerApiClient {
    fn new() -> Self {
        Self {}
    }

    async fn send_message(
        &self,
        message_hash_id: String,
        text: String,
    ) -> Result<(), MessengerExtError> {
        // Placeholder implementation. Replace with actual Messenger API message sending logic.
        Ok(())
    }
}
