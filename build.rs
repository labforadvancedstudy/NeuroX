fn main() {
    tonic_build::compile_protos("proto/neuron.proto").unwrap();
    tonic_build::compile_protos("proto/supervisor.proto").unwrap();
    tonic_build::compile_protos("proto/eye_ext.proto").unwrap();
    tonic_build::compile_protos("proto/webhook_ext.proto").unwrap();
    tonic_build::compile_protos("proto/messenger_ext.proto").unwrap();
}
