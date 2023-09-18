mod sdl;
mod cpu;
mod util;

use sdl::events::GBButton;
use self::cpu::memory;

fn main() {
    let mut handles = sdl::SdlHandles::new();
    
    loop {
        handles.events.update_events();
        handles.canvas.update();
    }
}
