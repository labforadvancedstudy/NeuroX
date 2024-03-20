# NeuroX by Lab for Advanced Study (WIP)

NeuroX is an innovative distributed neural network system designed to simulate the human brain by processing various types of input data, learning from the data, and generating outputs. The system architecture is based on a network of interconnected Neuron services that can be extended with specialized input processing capabilities using extension modules. NeuroX aims to provide a scalable and fault-tolerant solution for building and deploying artificial intelligence applications that mimic the functioning of the human brain, taking into account the complex interactions between different brain regions and the role of neurotransmitters in modulating neural activity.

You can find the detailed paper on NeuroX [here](https://github.com/labforadvancedstudy/paper/blob/main/NeuroX.md).

# WORK IN PROGRESS


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
