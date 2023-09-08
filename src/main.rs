mod sdl;
mod cpu;

use sdl::events::GBButton;

fn main() {
    let mut handles = sdl::SdlHandles::new();
    
    loop {
        handles.events.update_events();
        handles.canvas.update();
    }
}
