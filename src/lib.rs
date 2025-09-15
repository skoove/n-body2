use std::time::Duration;

use log::info;
use sdl3::{
    event::{Event, EventPollIterator},
    pixels::Color,
};

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

    'running: loop {
        state.i = (state.i + 1) % 255;

        if state.should_quit {
            break 'running;
        }

        state.handle_events(event_pump.poll_iter());
        state.render(&mut canvas);
    }

    info!("goodbye!")
}

#[derive(Default)]
struct SimulationState {
    should_quit: bool,
    i: u8,
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
        canvas.set_draw_color(Color::RGB(self.i, 100, 255 - self.i));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
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
        canvas.present();

        std::thread::sleep(Duration::from_millis(8));
    }
}
