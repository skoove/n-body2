use std::time::Duration;

use log::info;
use sdl3::{
    event::{Event, EventPollIterator},
    pixels::Color,
};

use math::Vec2;

mod math;

pub fn run() {
    let sdl3_context = sdl3::init().unwrap();
    let mut event_pump = sdl3_context.event_pump().unwrap();
    let window = sdl3_context
        .video()
        .unwrap()
        .window("n-body-2", 1000, 1000)
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();

    let mut state = SimulationState::default();

    let particles: Vec<Particle> = vec![
        Particle {
            position: Vec2::new(100.0, 300.0),
            veloctiy: Vec2::default(),
            acceleration: Vec2::default(),
        },
        Particle {
            position: Vec2::new(200.0, 300.0),
            veloctiy: Vec2::default(),
            acceleration: Vec2::default(),
        },
        Particle {
            position: Vec2::new(300.0, 300.0),
            veloctiy: Vec2::default(),
            acceleration: Vec2::default(),
        },
        Particle {
            position: Vec2::new(400.0, 300.0),
            veloctiy: Vec2::default(),
            acceleration: Vec2::default(),
        },
    ];

    state.particles = particles;

    'running: loop {
        if state.should_quit {
            break 'running;
        }

        state.handle_events(event_pump.poll_iter());
        state.render(&mut canvas);

        for particle in state.particles.iter_mut() {
            particle.acceleration = Vec2::new(0.01, 0.04);
            particle.veloctiy += particle.acceleration;
            particle.position += particle.veloctiy
        }
    }

    info!("goodbye!")
}

#[derive(Default)]
struct SimulationState {
    should_quit: bool,
    particles: Vec<Particle>,
}

#[derive(Default, Clone, Copy)]
struct Particle {
    position: Vec2,
    veloctiy: Vec2,
    acceleration: Vec2,
}

impl SimulationState {
    fn handle_events(&mut self, events: EventPollIterator) {
        for event in events {
            match event {
                Event::Quit { .. } => self.should_quit = true,
                _ => (),
            }
        }
    }

    fn render(&self, canvas: &mut sdl3::render::Canvas<sdl3::video::Window>) {
        canvas.set_draw_color((000, 000, 000));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let size = canvas.output_size().unwrap();
        let center_x = size.0 / 2;
        let center_y = size.1 / 2;
        let rect_size = 200.0;

        canvas
            .draw_rect(sdl3::render::FRect {
                x: center_x as f32 - (rect_size / 2.0),
                y: center_y as f32 - (rect_size / 2.0),
                w: rect_size,
                h: rect_size,
            })
            .unwrap();

        for particle in self.particles.iter() {
            canvas
                .draw_rect(sdl3::render::FRect {
                    x: particle.position.x - 1.0,
                    y: particle.position.y - 1.0,
                    w: 3.0,
                    h: 3.0,
                })
                .unwrap();
        }

        canvas.present();

        std::thread::sleep(Duration::from_millis(8));
    }
}
