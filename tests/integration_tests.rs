// tests/integration_tests.rs
use neurox::proto::neuron_service_client::NeuronServiceClient;
use neurox::proto::{InputSignal, OutputSignal, WeightUpdate};
use tonic::Request;

#[tokio::test]
async fn test_neuron_service() {
    let mut client = NeuronServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    let input_signal = InputSignal {
        values: vec![1.0; 10],
    };
    let request = Request::new(input_signal);
    let response = client.process_input(request).await.unwrap();
    let output_signal = response.into_inner();

    assert!(output_signal.value > 0.0);

    let weight_update = WeightUpdate {
        deltas: vec![0.1; 10],
    };
    let request = Request::new(weight_update);
    client.update_weights(request).await.unwrap();

    let input_signal = InputSignal {
        values: vec![1.0; 10],
    };
    let request = Request::new(input_signal);
    let response = client.process_input(request).await.unwrap();
    let output_signal = response.into_inner();

    assert!(output_signal.value > 0.0);
}