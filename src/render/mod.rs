use glam::Vec2;

pub mod sdl_software_renderer;
pub mod wgpu_renderer;

/// Kinds of things the renderer can do, send these to a [`Renderer`] and it will
/// render them
#[derive(Clone, Copy, Debug)]
pub enum RenderInstruction {
    Circle { position: Vec2, radius: f32 },
}

/// A renderer is a struct that can take in a list of [`RenderInstruction`] and
/// will draw a frame. For a simple example, see the [`SDLSoftwareRenderer`]
pub trait Renderer {
    fn render(&mut self, instructions: &[RenderInstruction]);
}
