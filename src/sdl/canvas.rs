extern crate sdl2;

use sdl2::{video::Window, pixels::Color, rect::Rect};

use super::PIXEL_SIZE;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Pixel {
    LightestGreen,
    LightGreen,
    DarkGreen,
    DarkestGreen,
}

pub struct CanvasUtils {
    handle: sdl2::render::Canvas<Window>,
    pixel_data: [[Pixel; 160]; 144],
}

impl CanvasUtils {
    
    pub fn new(window: Window) -> Self {
        let handle = window.into_canvas()
            .build()
            .unwrap();

        let pixel_data = [[Pixel::LightestGreen; 160]; 144];

        Self {
            handle,
            pixel_data,
        }
    }

    pub fn clear_screen(&mut self) {
        self.handle.set_draw_color(Color::RGB(0, 0, 0));
        self.handle.clear();
    }


    pub fn update(&mut self) {
        for y in 0..self.pixel_data.len() {
            for x in 0..self.pixel_data[y].len() {
                
                match self.pixel_data[y][x] {
                    Pixel::LightestGreen => { self.handle.set_draw_color(Color::RGB(0x9B, 0xBC, 0x0F))},
                    Pixel::LightGreen => { self.handle.set_draw_color(Color::RGB(0x8B, 0xAC, 0x0F))},
                    Pixel::DarkGreen => { self.handle.set_draw_color(Color::RGB(0x30, 0x62, 0x30))},
                    Pixel::DarkestGreen => { self.handle.set_draw_color(Color::RGB(0x0F, 0x38, 0x0F))},
                }
                
                self.handle.fill_rect(Rect::new(
                        (x as u32*PIXEL_SIZE) as i32, 
                        (y as u32*PIXEL_SIZE) as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE
                    )).unwrap();

            }
        }
        self.handle.present();
    }
}
