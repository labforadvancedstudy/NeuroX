// proto/mod.rs
pub mod neuron {
    tonic::include_proto!("neuron");
    }
    
    pub mod supervisor {
    tonic::include_proto!("supervisor");
    }
    
    pub mod eye_ext {
    tonic::include_proto!("eye_ext");
    }
    
    pub mod webhook_ext {
    tonic::include_proto!("webhook_ext");
    }
    
    pub mod messenger_ext {
    tonic::include_proto!("messenger_ext");
    }
//     