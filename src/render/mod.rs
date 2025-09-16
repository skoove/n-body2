use glam::Vec2;

pub mod sdl_software_renderer;
pub mod wgpu_renderer;

/// Kinds of things the renderer can do, send these to a [`Renderer`] and it will
/// render them
#[derive(Clone, Copy, Debug)]
pub enum RenderInstruction {
    Circle { position: Vec2, radius: f32 },
}

pub struct Camera {
    pub position: Vec2,
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            scale: 1.0,
        }
    }
}
