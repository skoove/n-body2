use crate::render::{RenderInstruction, Renderer};

pub struct WGPURenderer {}

impl Renderer for WGPURenderer {
    fn render(&mut self, instructions: &[RenderInstruction]) {
        for instruction in instructions {
            match instruction {
                RenderInstruction::Circle { position, radius } => todo!(),
            }
        }
    }
}

impl WGPURenderer {
    pub fn new() -> Self {
        Self {}
    }
}
