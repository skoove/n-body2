use glam::Vec2;
use sdl3::render::FPoint;

/// Kinds of things the renderer can do, send these to the renderer and it will
/// render them
#[derive(Clone, Copy, Debug)]
pub enum RenderInstruction {
    Circle { position: Vec2, radius: f32 },
}

pub struct Renderer {
    canvas: sdl3::render::Canvas<sdl3::video::Window>,
    pub camera: Camera,
}

pub struct Camera {
    pub position: Vec2,
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            scale: 1.0,
        }
    }
}

impl Renderer {
    pub fn new(canvas: sdl3::render::Canvas<sdl3::video::Window>) -> Self {
        Self {
            canvas,
            camera: Camera::default(),
        }
    }

    pub fn render(&mut self, instructions: &[RenderInstruction]) {
        self.canvas.set_draw_color((000, 000, 000));
        self.canvas.clear();
        self.canvas.set_draw_color((255, 255, 255));

        for instruction in instructions.iter() {
            match instruction {
                RenderInstruction::Circle { position, radius } => {
                    let position = self.world_to_screen(*position);
                    let mut points = Vec::<Vec2>::new();
                    let radius = (*radius * self.camera.scale) as i32;
                    let radius = if radius == 0 { 1 } else { radius };
                    for y in -radius..radius {
                        for x in -radius..=radius {
                            if x * x + y * y <= radius * radius {
                                points
                                    .push(Vec2::new(x as f32 + position.x, y as f32 + position.y));
                            };
                        }
                    }

                    let points: Vec<FPoint> =
                        points.iter().map(|p| FPoint::new(p.x, p.y)).collect();

                    self.canvas.draw_points(&*points).unwrap();
                }
            }
        }

        self.canvas.present();
    }

    fn world_to_screen(&self, world_coord: Vec2) -> Vec2 {
        let (w, h) = self.canvas.output_size().unwrap();
        let screen_center = Vec2::new(w as f32 / 2.0, h as f32 / 2.0);

        let translated = world_coord - self.camera.position;
        let scaled = translated * self.camera.scale;

        scaled + screen_center
    }
}
