// extensions/eye_ext.rs
use crate::proto::eye_ext_server::{EyeExt as EyeExtTrait, EyeExtServer};
use crate::proto::{EyeExtRequest, EyeExtResponse};
use std::convert::Infallible;
use thiserror::Error;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Error, Debug)]
pub enum EyeExtError {
    #[error("Channel send error: {0}")]
    ChannelSend(#[from] mpsc::error::SendError<Vec<f32>>),
    #[error("Image processing error: {0}")]
    ImageProcessing(String),
}

pub struct EyeExt {
    sender: mpsc::Sender<Vec<f32>>,
}

impl EyeExt {
    pub fn new(sender: mpsc::Sender<Vec<f32>>) -> Self {
        Self { sender }
    }
}

#[tonic::async_trait]
impl EyeExtTrait for EyeExt {
    async fn process_image(
        &self,
        request: Request<EyeExtRequest>,
    ) -> Result<Response<EyeExtResponse>, Status> {
        let EyeExtRequest { image_data } = request.into_inner();
        let tokens = self.preprocess_and_tokenize(image_data)?;
        self.sender.send(tokens).await?;
        Ok(Response::new(EyeExtResponse {}))
    }
}

impl EyeExt {
    fn preprocess_and_tokenize(&self, image_data: Vec<u8>) -> Result<Vec<f32>, EyeExtError> {
        // Placeholder implementation. Replace with actual image preprocessing and tokenization logic.
        Ok(vec![])
    }
}
