// telegram_bot.rs
use crate::proto::supervisor_client::SupervisorClient;
use crate::proto::SupervisorRequest;
use std::env;
use telegram_bot::*;
use tokio::sync::mpsc;
use tonic::Request;

pub struct TelegramBot {
api: Api,
receiver: mpsc::Receiver<(String, String)>,
chat_id: ChatId,
}

impl TelegramBot {
pub fn new(token: String, receiver: mpsc::Receiver<(String, String)>) -> Result<Self, Error> {
let api = Api::new(token);
let chat_id: i64 = env::var("TELEGRAM_CHAT_ID")
.expect("TELEGRAM_CHAT_ID not set")
.parse()
.expect("Failed to parse TELEGRAM_CHAT_ID");
let chat_id = ChatId::new(chat_id);
Ok(Self {
api,
receiver,
chat_id,
})
}


Copy code
pub async fn run(&mut self) {
    while let Some((command, response)) = self.receiver.recv().await {
        self.send_message(&command, &response)
            .await
            .expect("Failed to send Telegram message");
    }
}

async fn send_message(&self, command: &str, response: &str) -> Result<(), Error> {
    self.api
        .send(SendMessage::new(self.chat_id, response.to_string()))
        .await?;
    Ok(())
}
}
