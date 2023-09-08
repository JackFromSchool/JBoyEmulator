extern crate sdl2;

use sdl2::{event::Event, EventPump, Sdl, keyboard::Keycode};

pub struct EventHandler {
    event_pump: EventPump,
    pub events: Vec<GBButton>,
}

#[derive(PartialEq, Eq)]
pub enum GBButton {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

macro_rules! keycode_object{
    ($code:pat) => {
        Event::KeyDown {
            keycode: Some($code),
            ..
        }
    }
}

impl EventHandler {
    
    pub fn new(sdl_context: &Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            event_pump,
            events: Vec::new(),
        }
    }

    pub fn is_pressed(&self, button: GBButton) -> bool {
        self.events.contains(&button)
    }

    pub fn update_events(&mut self) {
        self.events.clear();

        for event in self.event_pump.poll_iter() {

            let found_code: Option<GBButton> = match event {
                keycode_object!(Keycode::Z) => Some(GBButton::A),
                keycode_object!(Keycode::X) => Some(GBButton::B),
                keycode_object!(Keycode::Up) => Some(GBButton::Up),
                keycode_object!(Keycode::Down) => Some(GBButton::Down),
                keycode_object!(Keycode::Left) => Some(GBButton::Left),
                keycode_object!(Keycode::Right) => Some(GBButton::Right),
                keycode_object!(Keycode::KpEnter) => Some(GBButton::Start),
                keycode_object!(Keycode::RShift) => Some(GBButton::Select),

                Event::Quit { .. } => {
                    std::process::exit(0);
                }

                _ => {None}
            }; 

            if found_code.is_some() {
                self.events.push(found_code.unwrap());
            }
        }
    }

}
