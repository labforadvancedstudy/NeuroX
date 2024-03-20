// extensions/webhook_ext.rs
use crate::proto::webhook_ext_server::{WebhookExt as WebhookExtTrait, WebhookExtServer};
use crate::proto::{WebhookExtRequest, WebhookExtResponse};
use std::convert::Infallible;
use thiserror::Error;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Error, Debug)]
pub enum WebhookExtError {
    #[error("Channel send error: {0}")]
    ChannelSend(#[from] mpsc::error::SendError<Vec<f32>>),
    #[error("JSON processing error: {0}")]
    JsonProcessing(String),
}

pub struct WebhookStreamExt {
    sender: mpsc::Sender<Vec<f32>>,
}

impl WebhookStreamExt {
    pub fn new(sender: mpsc::Sender<Vec<f32>>) -> Self {
        Self { sender }
    }
}

#[tonic::async_trait]
impl WebhookExtTrait for WebhookStreamExt {
    async fn process_json(
        &self,
        request: Request<WebhookExtRequest>,
    ) -> Result<Response<WebhookExtResponse>, Status> {
        let WebhookExtRequest { json_data } = request.into_inner();
        let tokens = self.parse_and_tokenize(json_data)?;
        self.sender.send(tokens).await?;
        Ok(Response::new(WebhookExtResponse {}))
    }
}

impl WebhookStreamExt {
    fn parse_and_tokenize(&self, json_data: String) -> Result<Vec<f32>, WebhookExtError> {
        // Placeholder implementation. Replace with actual JSON parsing and tokenization logic.
        Ok(vec![])
    }
}
