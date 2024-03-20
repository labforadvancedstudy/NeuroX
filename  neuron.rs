// neuron.rs
use crate::activation::Activation;
use crate::database::NeuronDb;
use crate::extensions::{EyeExt, WebhookStreamExt, MessengerInExt, MessengerOutExt};
use crate::proto::neuron_service_server::NeuronService;
use crate::proto::supervisor_client::SupervisorClient;
use crate::proto::{InputSignal, OutputSignal, SupervisorRequest};
use crate::weight_init::WeightInitializer;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

pub struct Neuron {
    id: String,
    weights: Vec<f32>,
    activation: Box<dyn Activation>,
    db: NeuronDb,
    eye_ext: Option<EyeExt>,
    webhook_ext: Option<WebhookStreamExt>,
    messenger_in_ext: Option<MessengerInExt>,
    messenger_out_ext: Option<MessengerOutExt>,
    extension_receiver: Option<mpsc::Receiver<Vec<f32>>>,
}

impl Neuron {
    pub fn new(
        id: String,
        num_inputs: usize,
        activation: Box<dyn Activation>,
        weight_initializer: &dyn WeightInitializer,
        eye_ext: Option<EyeExt>,
        webhook_ext: Option<WebhookStreamExt>,
        messenger_in_ext: Option<MessengerInExt>,
        messenger_out_ext: Option<MessengerOutExt>,
        extension_receiver: Option<mpsc::Receiver<Vec<f32>>>,
    ) -> Self {
        let mut weights = vec![0.0; num_inputs];
        weight_initializer.initialize(&mut weights);
        let db = NeuronDb::new(&id).expect("Failed to create neuron database");
        Self {
            id,
            weights,
            activation,
            db,
            eye_ext,
            webhook_ext,
            messenger_in_ext,
            messenger_out_ext,
            extension_receiver,
        }
    }

    async fn report_status(&self, status: String) {
        let mut client = SupervisorClient::connect("http://[::1]:50052")
            .await
            .expect("Failed to connect to supervisor");
        let request = Request::new(SupervisorRequest {
            neuron_id: self.id.clone(),
            status,
            metrics: "".to_string(),
            command: "".to_string(),
            args: vec![],
        });
        client
            .report_neuron_status(request)
            .await
            .expect("Failed to report neuron status");
    }

    async fn report_metrics(&self, metrics: HashMap<String, f64>) {
        let mut client = SupervisorClient::connect("http://[::1]:50052")
            .await
            .expect("Failed to connect to supervisor");
        let metrics_json = serde_json::to_string(&metrics).expect("Failed to serialize metrics");
        let request = Request::new(SupervisorRequest {
            neuron_id: self.id.clone(),
            status: "".to_string(),
            metrics: metrics_json,
            command: "".to_string(),
            args: vec![],
        });
        client
            .report_neuron_metrics(request)
            .await
            .expect("Failed to report neuron metrics");
    }

    fn calculate_metrics(&self, processing_time: f64) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("processing_time".to_string(), processing_time);
        // Placeholder implementation. Replace with actual metric calculation logic.
        metrics
    }

    async fn process_extensions(&mut self) -> Result<Vec<f32>, Status> {
        let mut input_tokens = Vec::new();
        while let Some(tokens) = self
            .extension_receiver
            .as_mut()
            .expect("Extension receiver not set")
            .recv()
            .await
        {
            input_tokens.extend(tokens);
        }
        Ok(input_tokens)
    }

    async fn process_messenger_in(&mut self, message_hash_id: String, text: String) -> Result<(), Status> {
        if let Some(ext) = &self.messenger_in_ext {
            ext.process_message(message_hash_id, text)
                .await
                .map_err(|e| {
                    log::error!("MessengerInExt error: {}", e);
                    Status::internal("Internal server error")
                })?;
        }
        Ok(())
    }

    async fn process_messenger_out(&mut self, message_hash_id: String, text: String) -> Result<(), Status> {
        if let Some(ext) = &self.messenger_out_ext {
            ext.send_message(message_hash_id, text).await.map_err(|e| {
                log::error!("MessengerOutExt error: {}", e);
                Status::internal("Internal server error")
            })?;
        }
        Ok(())
    }
}

#[tonic::async_trait]
impl NeuronService for Neuron {
    async fn process_input(
        &self,
        request: Request<InputSignal>,
    ) -> Result<Response<OutputSignal>, Status> {
        let start_time = Instant::now();
        let mut input = request.into_inner();
        self.report_status("Processing input".to_string()).await;

        let extension_tokens = self.process_extensions().await?;
        input.values.extend(extension_tokens);

        let mut z = 0.0;
        for (i, &value) in input.values.iter().enumerate() {
            z += value * self.weights[i];
        }
        let activation = self.activation.apply(z);

        self.db
            .put(b"activation", &activation.to_ne_bytes())
            .map_err(|e| {
                log::error!("Failed to store activation: {}", e);
                Status::internal("Internal server error")
            })?;

        let output = OutputSignal {
            value: activation,
        };

        let end_time = Instant::now();
        let processing_time = end_time.duration_since(start_time).as_secs_f64();
        let metrics = self.calculate_metrics(processing_time);
        self.report_metrics(metrics).await;
        self.report_status("Idle".to_string()).await;

        Ok(Response::new(output))
    }

    async fn update_weights(
        &self,
        request: Request<WeightUpdate>,
    ) -> Result<Response<()>, Status> {
        let WeightUpdate { deltas } = request.into_inner();

        for (i, &delta) in deltas.iter().enumerate() {
            self.weights[i] += delta;
        }

        self.db
            .put(b"weights", &self.weights.iter().map(|&w| w.to_ne_bytes()).flatten().collect::<Vec<_>>())
            .map_err(|e| {
                log::error!("Failed to store weights: {}", e);
                Status::internal("Internal server error")
            })?;

        Ok(Response::new(()))
    }
}