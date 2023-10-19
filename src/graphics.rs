use crate::sdl::SdlHandles;
use crate::sdl::canvas::Pixel;
use crate::cpu::Cpu;
use crate::util::BitGrabber;

pub struct Gpu {
    handle: SdlHandles,
    pixels: [[Pixel; 256]; 256],
    lcd: Lcd,
}

pub struct Lcd {
    pub enabled: bool,
    pub display_select: bool,
    pub window_enabled: bool,
    pub tile_data: bool,
    pub background_select: bool,
    pub sprite_size: bool,
    pub sprites_enabled: bool,
    pub background_enabled: bool,
}


impl Lcd {

    pub fn new() -> Self {
        Self {
            enabled: false,
            display_select: false,
            window_enabled: false,
            tile_data: false,
            background_select: false,
            sprite_size: false,
            sprites_enabled: false,
            background_enabled: false,
        }
    }
    
    pub fn update_with_byte(&mut self, byte: u8) {
        self.enabled = byte.nth_bit_as_bool(7);
        self.display_select = byte.nth_bit_as_bool(6);
        self.window_enabled = byte.nth_bit_as_bool(5);
        self.tile_data = byte.nth_bit_as_bool(4);
        self.background_select = byte.nth_bit_as_bool(3);
        self.sprite_size = byte.nth_bit_as_bool(2);
        self.sprites_enabled = byte.nth_bit_as_bool(1);
        self.background_enabled = byte.nth_bit_as_bool(0);
    }

}

impl Gpu {

    pub fn new(handle: SdlHandles) -> Self {
        Self {
            handle,
            pixels: [[Pixel::LightestGreen; 256]; 256],
            lcd: Lcd::new(),
        }
    }

    pub fn redraw(cpu: &Cpu) {
        
    }
}
