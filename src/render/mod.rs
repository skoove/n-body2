use bytemuck::{Pod, Zeroable};
use glam::Vec2;

pub mod wgpu_renderer;

#[derive(Clone, Copy, Debug)]
pub enum RenderInstruction {
    Circle { position: Vec2, radius: f32 },
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec2,
    pub scale: f32,
    pub x: u16,
    pub y: u16,
}

unsafe impl Pod for Camera {}
unsafe impl Zeroable for Camera {}
