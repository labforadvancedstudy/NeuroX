// tests/neuron_tests.rs
use neurox::activation::ReLU;
use neurox::database::NeuronDb;
use neurox::extensions::{EyeExt, WebhookStreamExt, MessengerInExt, MessengerOutExt};
use neurox::neuron::Neuron;
use neurox::proto::{InputSignal, OutputSignal, WeightUpdate};
use neurox::weight_init::XavierUniform;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tonic::Request;

#[tokio::test]
async fn test_neuron_process_input() {
    let neuron_id = "test_neuron".to_string();
    let num_inputs = 10;
    let activation = Box::new(ReLU);
    let weight_initializer = XavierUniform;

    let (extension_sender, extension_receiver) = mpsc::channel(32);
    let neuron = Neuron::new(
        neuron_id,
        num_inputs,
        activation,
        &weight_initializer,
        None,
        None,
        None,
        None,
        Some(extension_receiver),
    );

    let input_signal = InputSignal {
        values: vec![1.0; num_inputs],
    };
    let request = Request::new(input_signal);
    let response = neuron.process_input(request).await.unwrap();
    let output_signal = response.into_inner();

    assert!(output_signal.value > 0.0);
}

#[tokio::test]
async fn test_neuron_update_weights() {
    let neuron_id = "test_neuron".to_string();
    let num_inputs = 10;
    let activation = Box::new(ReLU);
    let weight_initializer = XavierUniform;

    let (extension_sender, extension_receiver) = mpsc::channel(32);
    let neuron = Neuron::new(
        neuron_id,
        num_inputs,
        activation,
        &weight_initializer,
        None,
        None,
        None,
        None,
        Some(extension_receiver),
    );

    let weight_update = WeightUpdate {
        deltas: vec![0.1; num_inputs],
    };
    let request = Request::new(weight_update);
    neuron.update_weights(request).await.unwrap();

    let db = NeuronDb::new(&neuron_id).unwrap();
    let stored_weights: Vec<f32> = db
        .get(b"weights")
        .unwrap()
        .unwrap()
        .chunks(4)
        .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
        .collect();

    assert_eq!(stored_weights.len(), num_inputs);
    for &w in &stored_weights {
        assert!(w > 0.0);
    }
}