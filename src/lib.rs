
extern crate sdl2;
pub mod draw;
pub mod math;
pub mod model;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window, pixels::Color};
pub struct Renderer {
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
}
impl Renderer {
    pub fn new(window_name: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(window_name, width, height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().software().build().unwrap();
        canvas.set_draw_color(Color::BLACK);
        let event_pump = sdl_context.event_pump().unwrap();
        Renderer {
            canvas,
            event_pump,
        }
    }

    pub fn if_close_window(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                _ => (),
            }
        }
        false
    }

    pub fn render(&mut self){
        self.canvas.present();
    }

    pub fn clear(&mut self){
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

}
