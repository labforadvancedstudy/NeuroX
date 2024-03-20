// main.rs
mod activation;
mod database;
mod extensions;
mod messenger_api_client;
mod neuron;
mod proto;
mod supervisor;
mod telegram_bot;
mod weight_init;

use activation::ReLU;
use extensions::{EyeExt, WebhookStreamExt, MessengerInExt, MessengerOutExt};
use messenger_api_client::MessengerApiClient;
use neuron::Neuron;
use proto::neuron_service_server::NeuronServiceServer;
use std::env;
use supervisor::Supervisor;
use telegram_bot::TelegramBot;
use tokio::sync::mpsc;
use tonic::transport::Server;
use weight_init::XavierUniform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let neuron_id = env::var("NEURON_ID").unwrap_or_else(|_| "neuron_1".to_string());
    let num_inputs = env::var("NUM_INPUTS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap();
    let activation = Box::new(ReLU);
    let weight_initializer = XavierUniform;

    let (eye_ext_sender, eye_ext_receiver) = mpsc::channel(32);
    let eye_ext = Some(EyeExt::new(eye_ext_sender));

    let (webhook_ext_sender, webhook_ext_receiver) = mpsc::channel(32);
    let webhook_ext = Some(WebhookStreamExt::new(webhook_ext_sender));

    let (messenger_in_ext_sender, messenger_in_ext_receiver) = mpsc::channel(32);
    let messenger_in_ext = Some(MessengerInExt::new(messenger_in_ext_sender));

    let messenger_api_client = MessengerApiClient::new(
        env::var("MESSENGER_API_TOKEN").expect("MESSENGER_API_TOKEN not set"),
    );
    let messenger_out_ext = Some(MessengerOutExt::new(messenger_api_client));

    let (extension_sender, extension_receiver) = mpsc::channel(32);
    tokio::spawn(async move {
        tokio::select! {
            _ = handle_eye_ext(eye_ext_receiver, extension_sender.clone()) => {}
            _ = handle_webhook_ext(webhook_ext_receiver, extension_sender.clone()) => {}
        }
    });

    let neuron = Neuron::new(
        neuron_id,
        num_inputs,
        activation,
        &weight_initializer,
        eye_ext,
        webhook_ext,
        messenger_in_ext,
        messenger_out_ext,
        Some(extension_receiver),
    );

    let neuron_addr = env::var("NEURON_ADDR").unwrap_or_else(|_| "[::1]:50051".to_string());
    let neuron_addr = neuron_addr.parse().unwrap();

    tokio::spawn(async move {
        Server::builder()
            .add_service(NeuronServiceServer::new(neuron))
            .serve(neuron_addr)
            .await
            .unwrap();
    });

    let (neuron_status_sender, neuron_status_receiver) = mpsc::channel(100);
    let (telegram_bot_sender, telegram_bot_receiver) = mpsc::channel(100);

    let supervisor = Supervisor::new(neuron_status_sender, telegram_bot_sender);

    let supervisor_addr = env::var("SUPERVISOR_ADDR").unwrap_or_else(|_| "[::1]:50052".to_string());
    let supervisor_addr = supervisor_addr.parse().unwrap();

    tokio::spawn(async move {
        Server::builder()
            .add_service(supervisor::SupervisorServer::new(supervisor))
            .serve(supervisor_addr)
            .await
            .unwrap();
    });

    let telegram_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let telegram_bot = TelegramBot::new(telegram_token, telegram_bot_receiver).unwrap();

    telegram_bot.run().await;

    Ok(())
}

async fn handle_eye_ext(
    mut eye_ext_receiver: mpsc::Receiver<Vec<u8>>,
    extension_sender: mpsc::Sender<Vec<f32>>,
) {
    while let Some(image_data) = eye_ext_receiver.recv().await {
        let eye_ext = EyeExt::new(extension_sender.clone());
        if let Err(e) = eye_ext.process(image_data).await {
            log::error!("EyeExt processing error: {}", e);
        }
    }
}

async fn handle_webhook_ext(
    mut webhook_ext_receiver: mpsc::Receiver<String>,
    extension_sender: mpsc::Sender<Vec<f32>>,
) {
    while let Some(json_data) = webhook_ext_receiver.recv().await {
        let webhook_ext = WebhookStreamExt::new(extension_sender.clone());
        if let Err(e) = webhook_ext.process(json_data).await {
            log::error!("WebhookStreamExt processing error: {}", e);
        }
    }
}
