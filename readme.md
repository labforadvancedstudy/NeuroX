# NeuroX by Lab for Advanced Study

NeuroX is a distributed neural network system designed to process various types of input data, learn from the data, and generate outputs. It consists of a network of Neuron services that can be extended with specialized input processing capabilities using extension modules.

## Features

Distributed architecture for scalability and fault tolerance
Support for various input types (images, webhooks, messages)
Extensible design allowing for easy integration of new features
Real-time monitoring and management of Neuron services
User interaction through a Telegram bot interface

## Getting Started

Clone the repository: git clone https://github.com/lab-for-advanced-study/neurox.git
Configure the environment variables (e.g., NEURON_ID, TELEGRAM_BOT_TOKEN, etc.).
Build and run the services using Docker Compose: docker-compose up --build
Interact with the system using the Telegram bot or gRPC client.

## Testing

### Unit Tests
To run unit tests for NeuroX, use the following command:


```bash
cargo test
```

### Integration Tests
To run integration tests for NeuroX, use the following command:

```
cargo test --test integration_tests
```

### End-to-End Tests
To run end-to-end tests for NeuroX, follow these steps:

Start the NeuroX system using Docker Compose: docker-compose up
In a separate terminal, run the end-to-end tests: cargo test --test e2e_tests

### Test Environment
NeuroX provides a test environment for running tests and experimenting with the system. To set up the test environment, follow these steps:

Deploy the test environment using Kubernetes: kubectl apply -f k8s/test/
Access the Neuron and Supervisor services using the exposed service endpoints.
Run tests or interact with the system using the provided gRPC client or Telegram bot.
For detailed instructions on running tests and using the test environment, please refer to the Testing Guide.

## Documentation
### System Design
### System Specification
### API Reference

## Contributing
Contributions are welcome! Please read the contribution guidelines before getting started.

## License
This project is licensed under the MIT License.
