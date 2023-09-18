pub struct RegisterPair {
    pub left: u8,
    pub right: u8
}

impl RegisterPair {

    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
        }
    }

    pub fn take_as_one(&self) -> u16 {
        let top = (self.left as u16) << 8;
        top + self.right as u16
    }

    pub fn change_as_one(&mut self, new: u16) {
        self.left = ((new & 0xFF00) >> 8) as u8;
        self.right = (new & 0xFF) as u8;
    }

    pub fn is_zero_high(&self) -> bool {
        (self.right | 0b10000000) == self.right 
    }
    
    pub fn is_subtract_high(&self) -> bool {
        (self.right | 0b01000000) == self.right 
    }

    pub fn is_hcarry_high(&self) -> bool {
        (self.right | 0b00100000) == self.right 
    }

    pub fn is_carry_high(&self) -> bool {
        (self.right | 0b00010000) == self.right 
    }

    pub fn flip_zero_flag(&mut self) {
        self.right ^= 0b10000000;
    }
    
    pub fn flip_subtract_flag(&mut self) {
        self.right ^= 0b01000000;
    }

    pub fn flip_hcarry_flag(&mut self) {
        self.right ^= 0b00100000;
    }

    pub fn flip_carry_flag(&mut self) {
        self.right ^= 0b00010000;
    }

    pub fn flip_flags_down(&mut self) {
        if self.is_zero_high() {
            self.flip_zero_flag()
        }
        if self.is_carry_high() {
            self.flip_carry_flag()
        }
        if self.is_hcarry_high() {
            self.flip_hcarry_flag()
        }
        if self.is_subtract_high() {
            self.flip_subtract_flag()
        }
    }

}

pub struct Registers {
    pub af: RegisterPair,
    pub bc: RegisterPair,
    pub de: RegisterPair,
    pub hl: RegisterPair,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {

    pub fn new() -> Self {
        Self {
            af: RegisterPair::new(),
            bc: RegisterPair::new(),
            de: RegisterPair::new(),
            hl: RegisterPair::new(),
            sp: 0,
            pc: 0,
        }
    }

}
