
// weight_init.rs
pub trait WeightInitializer {
    fn initialize(&self, weights: &mut [f32]);
    }
    
    pub struct XavierUniform;
    
    impl WeightInitializer for XavierUniform {
    fn initialize(&self, weights: &mut [f32]) {
    let n = weights.len() as f32;
    let range = (6.0 / n).sqrt();
    let mut rng = rand::thread_rng();
    for weight in weights {
    *weight = rng.gen_range(-range..range);
    }
    }
    }
    