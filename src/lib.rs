#![allow(clippy::new_without_default)]
#![allow(clippy::needless_return)]

use std::{thread, time::Instant};

use glam::Vec2;
use log::{error, info};
use sdl3::event::{Event, EventPollIterator, WindowEvent};

use dust_bunny::{RenderCommands, Renderer};

pub fn run() {
    let sdl3_context = sdl3::init().unwrap();
    let mut event_pump = sdl3_context.event_pump().unwrap();
    let window = sdl3_context
        .video()
        .unwrap()
        .window("n-body-2", 1200, 1000)
        .resizable()
        .build()
        .unwrap();

    let mut program_state = ProgramState::new(window);

    let (tx, rx) = std::sync::mpsc::sync_channel::<RenderCommands>(0);

    std::thread::spawn(move || {
        let mut render_commands = RenderCommands::new();

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
                render_commands.draw_circle(pos, 50.0);
            }

            let send_result = tx.send(render_commands.clone());
            if send_result.is_err() {
                info!("assuming main dropped the receiver, goodbye from simulation thread");
                break 'running; // if it was an error, its probably because main dropped the reciever, so exit cleanly
            };

            render_commands.clear();

            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let mut render_commands = RenderCommands::new();

    let mut last_now = Instant::now();
    'running: loop {
        if program_state.should_quit {
            break 'running;
        }

        let now = Instant::now();
        program_state.delta_time = (now - last_now).as_secs_f64();
        last_now = now;

        program_state.handle_events(event_pump.poll_iter());

        if let Ok(new_render_commands) = rx.try_recv() {
            render_commands = new_render_commands;
        }

        match program_state.renderer.render(&render_commands) {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                program_state.renderer.resize();
            }
            Err(e) => {
                error!("failed to render, rip bozo: {}", e)
            }
        };
    }

    info!("goodbye!")
}

struct ProgramState {
    should_quit: bool,
    renderer: Renderer,
    delta_time: f64,
}

impl ProgramState {
    fn new(window: sdl3::video::Window) -> Self {
        Self {
            should_quit: false,
            renderer: pollster::block_on(Renderer::new(window)),
            delta_time: 0.0,
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
                        self.renderer.camera.position +=
                            Vec2::new(xrel, -yrel) * self.renderer.camera.scale; // flip y, i dont know y, ahayuhahahahahahahahaa (i do actually, its because y increases as you go down, which is opposite to how it normally is)
                    }
                }
                Event::MouseWheel {
                    y,
                    mouse_x,
                    mouse_y,
                    ..
                } => {
                    let cursor_coords = Vec2::new(mouse_x, mouse_y);
                    let before_coords = self.renderer.camera.cursor_to_world_coords(cursor_coords);

                    self.renderer.camera.scale -= y * 0.5 * self.renderer.camera.scale;
                    self.renderer.camera.scale = self.renderer.camera.scale.max(0.01);

                    let after_coords = self.renderer.camera.cursor_to_world_coords(cursor_coords);
                    self.renderer.camera.position += before_coords - after_coords;
                }

                Event::Window {
                    win_event: WindowEvent::Resized(_, _),
                    ..
                } => self.renderer.resize(),
                _ => (),
            }
        }
    }
}
