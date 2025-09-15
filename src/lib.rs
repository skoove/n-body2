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

    let mut program_state = ProgramState::default();
    let mut simulation_state = SimulationState::default();

    let (tx, rx) = std::sync::mpsc::sync_channel::<Point>(3);

    std::thread::spawn(move || {
        let mut i = 0u32;

        'running: loop {
            i += 1;
            let i = (i % 250) as f32 / 250.0 * std::f32::consts::TAU;
            let x = i.cos() * 250.0;
            let y = i.sin() * 250.0;

            let send_result = tx.send(Point {
                position: Vec2::new(x, y),
            });

            if send_result.is_err() {
                info!("assuming main dropped the receiver, goodbye from simulation thread");
                break 'running; // if it was an error, its probably because main dropped the reciever, so exit cleanly
            };

            std::thread::sleep(Duration::from_millis(100));
        }
    });

    'running: loop {
        if program_state.should_quit {
            break 'running;
        }

        program_state.handle_events(event_pump.poll_iter());
        simulation_state.render(&mut canvas);
        let new_point = rx.try_recv().ok();
        if let Some(new_point) = new_point {
            simulation_state.point = new_point
        }
    }

    info!("goodbye!")
}

#[derive(Default)]
struct ProgramState {
    should_quit: bool,
}

#[derive(Default)]
struct SimulationState {
    point: Point,
}

#[derive(Default, Clone, Copy)]
struct Point {
    position: Vec2,
}

impl ProgramState {
    fn handle_events(&mut self, events: EventPollIterator) {
        for event in events {
            match event {
                Event::Quit { .. } => self.should_quit = true,
                _ => (),
            }
        }
    }
}

impl SimulationState {
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

        canvas
            .draw_rect(sdl3::render::FRect {
                x: (self.point.position.x - 2.0) + canvas.output_size().unwrap().0 as f32 / 2.0,
                y: (self.point.position.y - 2.0) + canvas.output_size().unwrap().1 as f32 / 2.0,
                w: 5.0,
                h: 5.0,
            })
            .unwrap();

        canvas.present();

        std::thread::sleep(Duration::from_millis(8));
    }
}
