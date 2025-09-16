use std::{thread, time::Instant};

use glam::Vec2;
use log::info;
use sdl3::event::{Event, EventPollIterator};

use crate::render::{RenderInstruction, Renderer, sdl_software_renderer::SDLSoftwareRenderer};

pub mod render;

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

    let canvas = window.into_canvas();

    let mut program_state = ProgramState::new(canvas);

    let (tx, rx) = std::sync::mpsc::sync_channel::<Vec<RenderInstruction>>(0);

    std::thread::spawn(move || {
        let mut render_instructions: Vec<RenderInstruction> = vec![];

        let mut last_time = Instant::now();

        let mut angle = 0.0;

        'running: loop {
            let now = Instant::now();
            let dt = (now - last_time).as_secs_f32();
            last_time = now;

            angle += dt * 2.0;
            angle %= std::f32::consts::TAU;

            let orbit_radius = 250.0;
            let count = 5;

            let mut positions = vec![];

            for i in 0..count {
                let offset = (i as f32 / count as f32) * std::f32::consts::TAU;
                positions.push(Vec2::new(
                    (angle + offset).cos() * orbit_radius,
                    (angle + offset).sin() * orbit_radius,
                ));
            }

            for pos in positions {
                render_instructions.push(RenderInstruction::Circle {
                    position: pos,
                    radius: 50.0,
                });
            }

            let send_result = tx.send(render_instructions.clone());
            if send_result.is_err() {
                info!("assuming main dropped the receiver, goodbye from simulation thread");
                break 'running; // if it was an error, its probably because main dropped the reciever, so exit cleanly
            };

            render_instructions.clear();

            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let mut render_instructions = vec![];
    'running: loop {
        if program_state.should_quit {
            break 'running;
        }

        program_state.handle_events(event_pump.poll_iter());
        if let Ok(new_render_instructions) = rx.try_recv() {
            render_instructions = new_render_instructions;
        }

        program_state.renderer.render(&render_instructions);
    }

    info!("goodbye!")
}

struct ProgramState {
    should_quit: bool,
    renderer: SDLSoftwareRenderer,
}

impl ProgramState {
    fn new(canvas: sdl3::render::Canvas<sdl3::video::Window>) -> Self {
        Self {
            should_quit: false,
            renderer: SDLSoftwareRenderer::new(canvas),
        }
    }

    fn handle_events(&mut self, events: EventPollIterator) {
        for event in events {
            match event {
                Event::Quit { .. } => self.should_quit = true,
                Event::MouseMotion {
                    mousestate,
                    xrel,
                    yrel,
                    ..
                } => {
                    if mousestate.right() {
                        self.renderer.camera.position -=
                            Vec2::new(xrel, yrel) / self.renderer.camera.scale
                    }
                }
                Event::MouseWheel { y, .. } => {
                    self.renderer.camera.scale += y * 0.5 * self.renderer.camera.scale
                }
                _ => (),
            }
        }
    }
}
