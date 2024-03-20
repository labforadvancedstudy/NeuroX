// supervisor.rs
use crate::proto::supervisor_server::{Supervisor as SupervisorTrait, SupervisorServer};
use crate::proto::{
SupervisorRequest, SupervisorResponse, SupervisorStatusRequest, SupervisorStatusResponse,
SupervisorMetricsRequest, SupervisorMetricsResponse,
};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tonic::{transport::Server, Request, Response, Status};

pub struct Supervisor {
neuron_status_sender: mpsc::Sender<(String, String)>,
telegram_bot_sender: mpsc::Sender<(String, String)>,
neuron_status: HashMap<String, String>,
neuron_metrics: HashMap<String, HashMap<String, f64>>,
}

impl Supervisor {
pub fn new(
neuron_status_sender: mpsc::Sender<(String, String)>,
telegram_bot_sender: mpsc::Sender<(String, String)>,
) -> Self {
Self {
neuron_status_sender,
telegram_bot_sender,
neuron_status: HashMap::new(),
neuron_metrics: HashMap::new(),
}
}

async fn process_neuron_status(&mut self, neuron_id: String, status: String) {
    log::info!("Received status from Neuron {}: {}", neuron_id, status);
    self.neuron_status.insert(neuron_id, status);
}

async fn process_neuron_metrics(&mut self, neuron_id: String, metrics: HashMap<String, f64>) {
    log::info!("Received metrics from Neuron {}: {:?}", neuron_id, metrics);
    self.neuron_metrics.insert(neuron_id, metrics);
}

async fn process_telegram_command(&mut self, command: String, args: Vec<String>) {
    log::info!("Received Telegram command: {}", command);
    let response = match command.as_str() {
        "/neuron_status" => self.handle_neuron_status().await,
        "/neuron_metrics" => self.handle_neuron_metrics(args).await,
        "/help" => self.handle_help().await,
        _ => "Unknown command. Type /help for available commands.".to_string(),
    };
    self.telegram_bot_sender
        .send((command, response))
        .await
        .expect("Failed to send response to Telegram bot");
}

async fn handle_neuron_status(&mut self) -> String {
    if self.neuron_status.is_empty() {
        "No neuron status available.".to_string()
    } else {
        let mut status_messages = Vec::new();
        for (neuron_id, status) in &self.neuron_status {
            status_messages.push(format!("Neuron {}: {}", neuron_id, status));
        }
        status_messages.join("\n")
    }
}

async fn handle_neuron_metrics(&mut self, args: Vec<String>) -> String {
    if args.is_empty() {
        "Please provide a neuron ID. Usage: /neuron_metrics <neuron_id>".to_string()
    } else {
        let neuron_id = &args[0];
        if let Some(metrics) = self.neuron_metrics.get(neuron_id) {
            let mut metric_messages = Vec::new();
            for (metric_name, metric_value) in metrics {
                metric_messages.push(format!("{}: {}", metric_name, metric_value));
            }
            format!("Metrics for Neuron {}:\n{}", neuron_id, metric_messages.join("\n"))
        } else {
            format!("No metrics available for Neuron {}.", neuron_id)
        }
    }
}

async fn handle_help(&mut self) -> String {
    r#"Available commands:
/neuron_status - Get status of all Neurons
/neuron_metrics <neuron_id> - Get metrics of a specific Neuron
/help - Show this help message"#
.to_string()
}
}

#[tonic::async_trait]
impl SupervisorTrait for Supervisor {
async fn report_neuron_status(
&self,
request: Request<SupervisorRequest>,
) -> Result<Response<SupervisorResponse>, Status> {
let SupervisorRequest {
neuron_id,
status,
..
} = request.into_inner();
self.neuron_status_sender
.send((neuron_id, status))
.await
.expect("Failed to send neuron status");
Ok(Response::new(SupervisorResponse {}))
}


async fn report_neuron_metrics(
    &self,
    request: Request<SupervisorRequest>,
) -> Result<Response<SupervisorResponse>, Status> {
    let SupervisorRequest {
        neuron_id,
        metrics,
        ..
    } = request.into_inner();
    let metrics: HashMap<String, f64> =
        serde_json::from_str(&metrics).expect("Failed to deserialize metrics");
    self.neuron_metrics.insert(neuron_id, metrics);
    Ok(Response::new(SupervisorResponse {}))
}

async fn get_neuron_status(
    &self,
    _request: Request<SupervisorStatusRequest>,
) -> Result<Response<SupervisorStatusResponse>, Status> {
    let neuron_status = self.neuron_status.clone();
    Ok(Response::new(SupervisorStatusResponse { neuron_status }))
}

async fn get_neuron_metrics(
    &self,
    request: Request<SupervisorMetricsRequest>,
) -> Result<Response<SupervisorMetricsResponse>, Status> {
    let SupervisorMetricsRequest { neuron_id } = request.into_inner();
    let metrics = self
        .neuron_metrics
        .get(&neuron_id)
        .cloned()
        .unwrap_or_default();
    Ok(Response::new(SupervisorMetricsResponse { metrics }))
}

async fn process_telegram_command(
    &self,
    request: Request<SupervisorRequest>,
) -> Result<Response<SupervisorResponse>, Status> {
    let SupervisorRequest {
        command,
        args,
        ..
    } = request.into_inner();
    self.telegram_bot_sender
        .send((command.clone(), args.clone()))
        .await
        .expect("Failed to send Telegram command");
    Ok(Response::new(SupervisorResponse {}))
}
}

