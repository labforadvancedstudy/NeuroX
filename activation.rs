// activation.rs
use std::fmt::Debug;

pub trait Activation: Send + Sync + Debug {
    fn apply(&self, x: f32) -> f32;
}

#[derive(Debug)]
pub struct ReLU;

impl Activation for ReLU {
    fn apply(&self, x: f32) -> f32 {
        x.max(0.0)
    }
}

#[derive(Debug)]
pub struct Sigmoid;

impl Activation for Sigmoid {
    fn apply(&self, x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }
}
