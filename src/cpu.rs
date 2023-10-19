pub mod memory;
pub mod register;

use self::memory::Memory;
use self::register::Registers;
use crate::util::BitGrabber;

pub enum RegCode {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    SP,
    AF,
    PC, //Internal use only
    Const8(u8),
    Const16(u16)
}

pub enum CondCode {
    NZ,
    NC,
    Z,
    C,
    Always,
}

pub struct Cpu {
    pub memory: Memory,
    pub registers: Registers,
    interupts: bool,
    halted: bool,
}

impl Cpu {

    pub fn new_with_rom(rom: &Vec<u8>) -> Self {
        let mut registers = Registers::new();
        registers.sp = 0xFFFF;
        registers.pc = 0x100;

        let mut memory = Memory::new();

        for (i, byte) in rom.iter().enumerate() {
            memory[0x100 + i] = *byte;
        }

        Self {
            memory,
            registers,
            interupts: true,
            halted: false,
        }
    }

    pub fn new() -> Self {
        let mut registers = Registers::new();
        registers.sp = 0xFFFF;

        Self {
            memory: Memory::new(),
            registers,
            interupts: true,
            halted: false,
        }
    }

    pub fn current_pc_byte(&self) -> u8 {
        self.memory[self.registers.pc.into()]
    }

    pub fn increment_pc(&mut self) {
        self.registers.pc += 1;
    }

    pub fn decrement_pc(&mut self) {
        self.registers.pc -= 1;
    }
    
    pub fn get_16_pc(&mut self) -> u16 {
        let mut num: u16 = (self.current_pc_byte() as u16) << 8;
        self.increment_pc();
        num += (self.current_pc_byte() as u16);
        num
    }
    
    /*
     *  HALT Instruction
     *  Halts cpu until an interupt
     */
    pub fn halt(&mut self) {
        self.halted = true;
    }
    
    /*
     *  EI Instruction
     *  Enables cpu interupts
     */
    pub fn ei(&mut self) {
        self.interupts = true;
    }
    
    /*
     *  DI Instruction
     *  Disables cpu interupts
     */
    pub fn di(&mut self) {
        self.interupts = false;
    }
    
    /*  
     *  LD Instruction for u8 into u8
     *  Source register or byte it is pointing to if u16 is stored
     *  Target register or byte it is pointing to is set to source
     */
    pub fn load8(&mut self, target: RegCode, source: RegCode) {
        let val = match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.right,
            RegCode::E => self.registers.de.left,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::BC => self.memory[self.registers.bc.take_as_one().into()],
            RegCode::DE => self.memory[self.registers.de.take_as_one().into()],
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid RegCode used as source for load8")
        };

        match target {
            RegCode::A => self.registers.af.left = val,
            RegCode::B => self.registers.bc.left = val,
            RegCode::C => self.registers.bc.right = val,
            RegCode::D => self.registers.de.right = val,
            RegCode::E => self.registers.de.left = val,
            RegCode::H => self.registers.hl.left = val,
            RegCode::L => self.registers.hl.right = val,
            RegCode::BC => self.memory[self.registers.bc.take_as_one().into()] = val,
            RegCode::DE => self.memory[self.registers.de.take_as_one().into()] = val,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val,
            RegCode::Const8(_) => (),
            _ => panic!("Invalid RegCode used as target for load8")
        };
    }

    pub fn load16(&mut self, target: RegCode, source: RegCode) {
        let val = match source {
            RegCode::BC => self.registers.bc.take_as_one(),
            RegCode::DE => self.registers.de.take_as_one(),
            RegCode::HL => self.registers.hl.take_as_one(),
            RegCode::SP => self.registers.sp,
            RegCode::Const16(i) => i,
            _ => panic!("Invalid RegCode used as target for load16"),
        };

        match target {
            RegCode::BC => self.registers.bc.change_as_one(val),
            RegCode::DE => self.registers.de.change_as_one(val),
            RegCode::HL => self.registers.hl.change_as_one(val),
            RegCode::SP => self.registers.sp = val,
            RegCode::Const16(i) => {
                self.memory[i as usize] = ((val & 0xF0) >> 4) as u8;
                self.memory[(i+1) as usize] = (val & 0xF) as u8;
            },
            _ => panic!("Invalid RegCode used as target for load16"),
        };
    }

    pub fn load_weird(&mut self, target: RegCode, source: RegCode) {
        let val = match source {
            RegCode::Const16(i) => self.memory[i as usize],
            RegCode::Const8(i) => self.memory[0xFF00 + i as usize],
            RegCode::A => self.memory[self.registers.af.left.into()],
            _ => panic!("Invalid RegCode used as source for load weird")
        };

        match target {
            RegCode::Const16(i) => self.memory[i as usize] = val,
            RegCode::Const8(i) => self.memory[0xFF00 + i as usize] = val,
            RegCode::A => self.registers.af.left = val,
            _ => panic!("Invalid RegCode used as target for load weird"),
        }
    }
    
    /*
     *  INC instruction for u8 and u16
     *  target register or the memory it points to is incremented by 1
     */
    pub fn increment8(&mut self, target: RegCode) {
        match target {
            RegCode::A => self.registers.af.left += 1,
            RegCode::B => self.registers.bc.left += 1,
            RegCode::C => self.registers.bc.right += 1,
            RegCode::D => self.registers.de.left += 1,
            RegCode::E => self.registers.de.right += 1,
            RegCode::H => self.registers.hl.left += 1,
            RegCode::L => self.registers.hl.right += 1,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] += 1,
            _ => panic!("Invalid RegCode used as target for increment8"),
        }
    }

    pub fn increment16(&mut self, target: RegCode) {
        match target {
            RegCode::BC => self.registers.bc.change_as_one(self.registers.bc.take_as_one().wrapping_add(1)),
            RegCode::DE => self.registers.de.change_as_one(self.registers.de.take_as_one().wrapping_add(1)),
            RegCode::HL => self.registers.hl.change_as_one(self.registers.hl.take_as_one().wrapping_add(1)),
            RegCode::SP => self.registers.sp = self.registers.sp.wrapping_add(1),
            _ => panic!("Invalid RegCode used as target for increment16")
        }
    }
    
    /*
     *  DEC instruction for u8 and u16
     *  target register or the memory it points to are decremented by 1
     */
    pub fn decrement8(&mut self, target: RegCode) {
        match target {
            RegCode::A => self.registers.af.left = self.registers.af.left.wrapping_sub(1),
            RegCode::B => self.registers.bc.left = self.registers.bc.left.wrapping_sub(1),
            RegCode::C => self.registers.bc.right = self.registers.bc.right.wrapping_sub(1),
            RegCode::D => self.registers.de.left = self.registers.de.left.wrapping_sub(1),
            RegCode::E => self.registers.de.right = self.registers.de.right.wrapping_sub(1),
            RegCode::H => self.registers.hl.left = self.registers.hl.left.wrapping_sub(1),
            RegCode::L => self.registers.hl.right = self.registers.hl.right.wrapping_sub(1),
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = self.memory[self.registers.hl.take_as_one().into()].wrapping_sub(1),
            _ => panic!("Invalid RegCode used as target for increment8"),
        };
    }

    pub fn decrement16(&mut self, target: RegCode) {
        match target {
            RegCode::BC => self.registers.bc.change_as_one(self.registers.bc.take_as_one().wrapping_sub(1)),
            RegCode::DE => self.registers.de.change_as_one(self.registers.de.take_as_one().wrapping_sub(1)),
            RegCode::HL => self.registers.hl.change_as_one(self.registers.hl.take_as_one().wrapping_sub(1)),
            RegCode::SP => self.registers.sp = self.registers.sp.wrapping_sub(1),
            _ => panic!("Invalid RegCode used as target for increment16")
        };
    }

    /*
     *  JR instruction
     *  CondCode is the condition that must be true for the jump
     *  jump_by is the relative distance to add to the program counter
     */
    pub fn jump_relative(&mut self, cond: CondCode, jump_by: i8) {
        let jump;

        match cond {
            CondCode::Z => jump = self.registers.af.is_zero_high(),
            CondCode::NZ => jump = !self.registers.af.is_zero_high(),
            CondCode::C => jump = self.registers.af.is_carry_high(),
            CondCode::NC => jump = !self.registers.af.is_carry_high(),
            CondCode::Always => jump = true,
        }

        if jump {
            self.registers.pc = (self.registers.pc as i16 + jump_by as i16) as u16;
        }
    }
    
    /*
     *  RLA instruction
     *  Rotates bits in register a left through the carry bit
     */
    pub fn rotate_left_carry_a(&mut self) {
        let carry_end_high = (self.registers.af.left | 0b10000000) == self.registers.af.left;
        let _ = self.registers.af.left <<= 1;
        if self.registers.af.is_carry_high() {
            self.registers.af.left += 1;
            self.registers.af.flip_carry_flag();
        }
        if carry_end_high && !self.registers.af.is_carry_high() {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_carry_flag();
        }
    }

    /*
     *  RLCA instruction
     *  Rotates bits in register a left not through the carry bit
     *  Sets the carry flag high if the last bit is high when rotated
     */
    pub fn rotate_left_a(&mut self) {
        let carry_end_high = (self.registers.af.left | 0b10000000) == self.registers.af.left;
        let _ = self.registers.af.left <<= 1;
        if carry_end_high && !self.registers.af.is_carry_high() {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_carry_flag();
        }
        if carry_end_high {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_carry_flag();
            self.registers.af.left += 1;
        }
    }
    
    /*
     *  RRA instruction
     *  Rotates bits in register a right through the carry bit
     */
    pub fn rotate_right_carry_a(&mut self) {
        let carry_end_high = (self.registers.af.left | 1) == self.registers.af.left;
        let _ = self.registers.af.left >>= 1;
        if self.registers.af.is_carry_high() {
            self.registers.af.left += 0b10000000;
            self.registers.af.flip_flags_down();
        }
        if carry_end_high && !self.registers.af.is_carry_high() {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_carry_flag();
        }
    }

    /*
     *  RRCA instruction
     *  Rotates bits in register a right not throught the carry bit
     *  Sets carry high if the first bit is high
     */
    pub fn rotate_right_a(&mut self) {
        let carry_end_high = (self.registers.af.left | 1) == self.registers.af.left;
        let _ = self.registers.af.left >>= 1;
        if carry_end_high && !self.registers.af.is_carry_high() {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_carry_flag();
        }
        if carry_end_high {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_carry_flag();
            self.registers.af.left += 0b10000000;
        }
    }
    
    /*
     *  ADD instruction for u8s
     *  Adds the register value to A
     */
    pub fn add8(&mut self, source: RegCode) {
        self.registers.af.flip_flags_down();

        let target_num = self.registers.af.left;
        let source_num = match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid RegCode used for add instruction"),
        };

        if target_num.nth_bit_as_bool(3) && source_num.nth_bit_as_bool(3) {
            self.registers.af.flip_hcarry_flag();
        }

        if target_num.nth_bit_as_bool(7) && target_num.nth_bit_as_bool(7) {
            self.registers.af.flip_carry_flag();
        }

        self.registers.af.left = self.registers.af.left.wrapping_add(source_num);
        if self.registers.af.left == 0 {
            self.registers.af.flip_zero_flag();
        }
    }

    /*
     *  ADD instruction for u16
     *  Adds the source value to hl register
     */
    pub fn add16(&mut self, source: RegCode) {
        if self.registers.af.is_zero_high() {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_zero_flag();
        } else {
            self.registers.af.flip_flags_down();
        }

        let target_num = self.registers.hl.take_as_one();
        let source_num = match source {
            RegCode::BC => self.registers.bc.take_as_one(),
            RegCode::DE => self.registers.de.take_as_one(),
            RegCode::HL => self.registers.hl.take_as_one(),
            RegCode::SP => self.registers.sp,
            _ => panic!("Invalid RegCode used for add instruction"),
        };

        if target_num.nth_bit_as_bool(11) && source_num.nth_bit_as_bool(11) {
            self.registers.af.flip_hcarry_flag();
        }

        if target_num.nth_bit_as_bool(15) && target_num.nth_bit_as_bool(15) {
            self.registers.af.flip_carry_flag();
        }

        self.registers.hl.change_as_one(self.registers.hl.take_as_one().wrapping_add(source_num));
    }
    
    /*
     *  ADD instruction for the sp register
     *  Takes val and adds it to sp
     */
    pub fn add_sp(&mut self, val: i8) {
        self.registers.af.flip_flags_down();
        if !val.is_negative() {
            if self.registers.sp.nth_bit_as_bool(3) && self.registers.sp.nth_bit_as_bool(3) {
                self.registers.af.flip_hcarry_flag();
            }
            if self.registers.sp.nth_bit_as_bool(3) && self.registers.sp.nth_bit_as_bool(3) {
                self.registers.af.flip_carry_flag();
            }
        }
        self.registers.sp = ((self.registers.sp as i16).wrapping_add(val as i16)) as u16;
    }
    
    /*
     *  SUB instruction
     *  Subtracts the source value from register A
     */
    pub fn sub(&mut self, source: RegCode) {
        self.registers.af.flip_flags_down();
        self.registers.af.flip_subtract_flag();
        let source_val = match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid Regcode used for subtraction")
        };
        let a_val = self.registers.af.left;
        if a_val < source_val {
            self.registers.af.flip_carry_flag();
        }
        if ((a_val & 0xF0).wrapping_sub((source_val & 0xF0) & 0x10)) == 0x10 {
            self.registers.af.flip_hcarry_flag();
        }
        if a_val.wrapping_sub(source_val) == 0 {
            self.registers.af.flip_zero_flag();
        }

        self.registers.af.left = self.registers.af.left.wrapping_sub(source_val);
    }
    
    /*
     *  AND instruction
     *  Ands the a register by the source value
     */
    pub fn and(&mut self, source: RegCode) {
        self.registers.af.flip_flags_down();
        self.registers.af.flip_hcarry_flag();
        self.registers.af.left &= match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid Regcode for and")
        };

        if self.registers.af.left == 0 {
            self.registers.af.flip_zero_flag();
        }
    }
    
    /*
     *  OR instruction
     *  Or's the a register by the source val
     */
    pub fn or(&mut self, source: RegCode) {
        self.registers.af.flip_flags_down();
        self.registers.af.left |= match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid Regcode for or")
        };

        if self.registers.af.left == 0 {
            self.registers.af.flip_zero_flag();
        }
    }

    /*
     *  XOR instruction
     *  Xor's the a register by the source val
     */
    pub fn xor(&mut self, source: RegCode) {
        self.registers.af.flip_flags_down();
        self.registers.af.left ^= match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid Regcode for xor")
        };

        if self.registers.af.left == 0 {
            self.registers.af.flip_zero_flag();
        }
    }

    /*
     *  ADC instruction
     *  Adds the source register value to a with the carry flag
     */
    pub fn addc(&mut self, source: RegCode) {
        let mut source_num: u8 = 0;
        if self.registers.af.is_carry_high() {
            source_num = 1;
        }
        self.registers.af.flip_flags_down();

        let target_num = self.registers.af.left;
        source_num = source_num.wrapping_add(match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid RegCode used for add instruction"),
        });

        if target_num.nth_bit_as_bool(3) && source_num.nth_bit_as_bool(3) {
            self.registers.af.flip_hcarry_flag();
        }

        if target_num.nth_bit_as_bool(7) && target_num.nth_bit_as_bool(7) {
            self.registers.af.flip_carry_flag();
        }

        self.registers.af.left = self.registers.af.left.wrapping_add(source_num);
        if self.registers.af.left == 0 {
            self.registers.af.flip_zero_flag();
        }
    }
    
    /*
     *  SBC instruction
     *  Subtracts the register value plus the carry from a
     */
    pub fn subc(&mut self, source: RegCode) {
        let mut source_val: u8 = 0;
        if self.registers.af.is_carry_high() {
            source_val = 1;
        }
        self.registers.af.flip_flags_down();
        self.registers.af.flip_subtract_flag();
        source_val = source_val.wrapping_add(match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid Regcode used for subtraction")
        });
        let a_val = self.registers.af.left;
        if a_val < source_val {
            self.registers.af.flip_carry_flag();
        }
        if (a_val & 0xF0).wrapping_sub((source_val & 0xF0) & 0x10) == 0x10 {
            self.registers.af.flip_hcarry_flag();
        }
        if a_val.wrapping_sub(source_val) == 0 {
            self.registers.af.flip_zero_flag();
        }

        self.registers.af.left = self.registers.af.left.wrapping_sub(source_val);
    }
    
    /*
     *  CP instruction
     *  Subtracts the register value from a and modifies flags acordingly
     *  Does not store that value in a afterwards
     */
    pub fn cp(&mut self, source: RegCode) {
        self.registers.af.flip_flags_down();
        self.registers.af.flip_subtract_flag();
        let source_val = match source {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            RegCode::Const8(i) => i,
            _ => panic!("Invalid Regcode used for subtraction")
        };
        let a_val = self.registers.af.left;
        if a_val < source_val {
            self.registers.af.flip_carry_flag();
        }
        if (a_val & 0xF0).wrapping_sub((source_val & 0xF0) & 0x10) == 0x10 {
            self.registers.af.flip_hcarry_flag();
        }
        if a_val.wrapping_sub(source_val) == 0 {
            self.registers.af.flip_zero_flag();
        }
    }

    pub fn push(&mut self, source: RegCode) {
        let source_val = match source {
            RegCode::BC => self.registers.bc.take_as_one(),
            RegCode::DE => self.registers.de.take_as_one(),
            RegCode::HL => self.registers.hl.take_as_one(),
            RegCode::AF => self.registers.af.take_as_one(),
            RegCode::PC => self.registers.pc,
            _ => panic!("Invalid RegCode for push"),
        };
        
        self.decrement16(RegCode::SP);
        self.memory[self.registers.sp.into()] = ((source_val & 0xFF00) >> 8) as u8;
        self.decrement16(RegCode::SP);
        self.memory[self.registers.sp.into()] = (source_val & 0xFF) as u8;
    }

    pub fn pop(&mut self, target: RegCode) {
        let mut val: u16 = self.memory[self.registers.sp.into()] as u16;
        self.increment16(RegCode::SP);
        val += (self.memory[self.registers.sp.into()] as u16) << 8;
        self.increment16(RegCode::SP);

        match target {
            RegCode::BC => self.registers.bc.change_as_one(val),
            RegCode::DE => self.registers.de.change_as_one(val),
            RegCode::HL => self.registers.hl.change_as_one(val),
            RegCode::AF => self.registers.af.change_as_one(val),
            RegCode::PC => self.registers.pc = val,
            _ => panic!("Invalid RegCode for push"),
        }
    }

    pub fn ret(&mut self, cond: CondCode) {
        if match cond {
            CondCode::Z => self.registers.af.is_zero_high(),
            CondCode::NZ => !self.registers.af.is_zero_high(),
            CondCode::C => self.registers.af.is_carry_high(),
            CondCode::NC => !self.registers.af.is_carry_high(),
            CondCode::Always => true,
        } {
            self.pop(RegCode::PC);
        }
    }

    pub fn jump(&mut self, cond: CondCode, to: u16) {
        if match cond {
            CondCode::Z => self.registers.af.is_zero_high(),
            CondCode::NZ => !self.registers.af.is_zero_high(),
            CondCode::C => self.registers.af.is_carry_high(),
            CondCode::NC => !self.registers.af.is_carry_high(),
            CondCode::Always => true,
        } {
            self.registers.pc = to;
            println!("Jumped to: {}", self.registers.pc);
        }
    }

    pub fn jump_hl(&mut self) {
        self.registers.pc = self.registers.hl.take_as_one();
    }

    pub fn call(&mut self, cond:CondCode, to: u16) {
        if match cond {
            CondCode::Z => self.registers.af.is_zero_high(),
            CondCode::NZ => !self.registers.af.is_zero_high(),
            CondCode::C => self.registers.af.is_carry_high(),
            CondCode::NC => !self.registers.af.is_carry_high(),
            CondCode::Always => true,
        } {
            self.push(RegCode::PC);
            self.registers.pc = to;
        }
    }

    pub fn restart(&mut self, to: u16) {
        if to != 0x10 || to != 0x20 || to != 0x30 || to != 0x08 || to != 0x18 || to != 0x28 || to != 0x38 || to != 0 {
            panic!("RST called with an invalid value: {to}");
        }
        self.push(RegCode::PC);
        self.registers.pc = to;
    }

    pub fn rotate_left_carry(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();
        
        let val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };

        if val.nth_bit_as_bool(7) {
            self.registers.af.flip_carry_flag();
        }

        if val == 0 {
            self.registers.af.flip_zero_flag();
        }

        match code {
            RegCode::A => self.registers.af.left = val.rotate_left(1),
            RegCode::B => self.registers.bc.left = val.rotate_left(1),
            RegCode::C => self.registers.bc.right = val.rotate_left(1),
            RegCode::D => self.registers.de.left = val.rotate_left(1),
            RegCode::E => self.registers.de.right = val.rotate_left(1),
            RegCode::H => self.registers.hl.left = val.rotate_left(1),
            RegCode::L => self.registers.hl.right = val.rotate_left(1),
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val.rotate_left(1),
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };
    }

    pub fn rotate_right_carry(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();
        
        let val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };

        if val.nth_bit_as_bool(0) {
            self.registers.af.flip_carry_flag();
        }

        if val == 0 {
            self.registers.af.flip_zero_flag();
        }

        match code {
            RegCode::A => self.registers.af.left = val.rotate_right(1),
            RegCode::B => self.registers.bc.left = val.rotate_right(1),
            RegCode::C => self.registers.bc.right = val.rotate_right(1),
            RegCode::D => self.registers.de.left = val.rotate_right(1),
            RegCode::E => self.registers.de.right = val.rotate_right(1),
            RegCode::H => self.registers.hl.left = val.rotate_right(1),
            RegCode::L => self.registers.hl.right = val.rotate_right(1),
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val.rotate_right(1),
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };
    }

    pub fn rotate_left(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();
        
        let mut val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };

        if val.nth_bit_as_bool(7) {
            self.registers.af.flip_carry_flag();
            val = val.rotate_left(1) ^ 0b00000001;
        } else {
            val = val.rotate_left(1);
        }

        if val == 0 {
            self.registers.af.flip_zero_flag();
        }

        match code {
            RegCode::A => self.registers.af.left = val,
            RegCode::B => self.registers.bc.left = val,
            RegCode::C => self.registers.bc.right = val,
            RegCode::D => self.registers.de.left = val,
            RegCode::E => self.registers.de.right = val,
            RegCode::H => self.registers.hl.left = val,
            RegCode::L => self.registers.hl.right = val,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val,
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };
    }

    pub fn rotate_right(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();
        
        let mut val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };

        if val.nth_bit_as_bool(0) {
            self.registers.af.flip_carry_flag();
            val = val.rotate_right(1) ^ 0b10000000; 
        } else {
            val = val.rotate_right(1);
        }

        if val == 0 {
            self.registers.af.flip_zero_flag();
        }

        match code {
            RegCode::A => self.registers.af.left = val,
            RegCode::B => self.registers.bc.left = val,
            RegCode::C => self.registers.bc.right = val,
            RegCode::D => self.registers.de.left = val,
            RegCode::E => self.registers.de.right = val,
            RegCode::H => self.registers.hl.left = val,
            RegCode::L => self.registers.hl.right = val,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val,
            _ => panic!("Rotate Left Carry called with invalid RegCode")
        };
    }

    pub fn shift_left(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();

        let mut val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Left shift called with invalid RegCode")
        };

        if val.nth_bit_as_bool(7) {
            self.registers.af.flip_carry_flag();
        }

        if val << 1 == 0 {
            self.registers.af.flip_zero_flag();
        }

        val <<= 1;

        match code {
            RegCode::A => self.registers.af.left = val,
            RegCode::B => self.registers.bc.left = val,
            RegCode::C => self.registers.bc.right = val,
            RegCode::D => self.registers.de.left = val,
            RegCode::E => self.registers.de.right = val,
            RegCode::H => self.registers.hl.left = val,
            RegCode::L => self.registers.hl.right = val,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val,
            _ => panic!("Left shift called with invalid RegCode")
        };
    }

    pub fn shift_right_arithmetic(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();

        let mut val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Right arithmetic shift called with invalid RegCode")
        };

        if val.nth_bit_as_bool(0) {
            self.registers.af.flip_carry_flag();
        }

        if val >> 1 == 0 {
            self.registers.af.flip_zero_flag();
        }

        val >>= 1;

        match code {
            RegCode::A => self.registers.af.left = val,
            RegCode::B => self.registers.bc.left = val,
            RegCode::C => self.registers.bc.right = val,
            RegCode::D => self.registers.de.left = val,
            RegCode::E => self.registers.de.right = val,
            RegCode::H => self.registers.hl.left = val,
            RegCode::L => self.registers.hl.right = val,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val,
            _ => panic!("Right arithmetic shift called with invalid RegCode")
        };
    }

    pub fn shift_right_logical(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();

        let mut val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Right logical shift called with invalid RegCode")
        };

        if val.nth_bit_as_bool(0) {
            self.registers.af.flip_carry_flag();
        }

        if val >> 1 == 0 { // FIX THIS LATER IT WILL PROBABLY BE BROKEN!!!!!!!!!
            self.registers.af.flip_zero_flag();
        }

        val >>= 1;

        match code {
            RegCode::A => self.registers.af.left = val,
            RegCode::B => self.registers.bc.left = val,
            RegCode::C => self.registers.bc.right = val,
            RegCode::D => self.registers.de.left = val,
            RegCode::E => self.registers.de.right = val,
            RegCode::H => self.registers.hl.left = val,
            RegCode::L => self.registers.hl.right = val,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val,
            _ => panic!("Right logical shift called with invalid RegCode")
        };
    }

    pub fn swap(&mut self, code: RegCode) {
        self.registers.af.flip_flags_down();

        let mut val = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("swap called with invalid RegCode")
        };

        if val.swap_bytes() == 0 { 
            self.registers.af.flip_zero_flag();
        }

        val = val.swap_bytes();

        match code {
            RegCode::A => self.registers.af.left = val,
            RegCode::B => self.registers.bc.left = val,
            RegCode::C => self.registers.bc.right = val,
            RegCode::D => self.registers.de.left = val,
            RegCode::E => self.registers.de.right = val,
            RegCode::H => self.registers.hl.left = val,
            RegCode::L => self.registers.hl.right = val,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] = val,
            _ => panic!("Swap called with invalid RegCode")
        };
    }

    pub fn bit_check_zero(&mut self, bit: usize, code: RegCode) {
        if self.registers.af.is_carry_high() {
            self.registers.af.flip_flags_down();
            self.registers.af.flip_carry_flag();
        } else {
            self.registers.af.flip_flags_down();
        }

        let to_check = match code {
            RegCode::A => self.registers.af.left,
            RegCode::B => self.registers.bc.left,
            RegCode::C => self.registers.bc.right,
            RegCode::D => self.registers.de.left,
            RegCode::E => self.registers.de.right,
            RegCode::H => self.registers.hl.left,
            RegCode::L => self.registers.hl.right,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Invalid RegCode for bit check")
        };

        if to_check.nth_bit_as_bool(bit) {
            self.registers.af.flip_zero_flag();
        }
    }

    pub fn bit_set(&mut self, bit: usize, code: RegCode) {
        let to_set = match code {
            RegCode::A => &mut self.registers.af.left,
            RegCode::B => &mut self.registers.bc.left,
            RegCode::C => &mut self.registers.bc.right,
            RegCode::D => &mut self.registers.de.left,
            RegCode::E => &mut self.registers.de.right,
            RegCode::H => &mut self.registers.hl.left,
            RegCode::L => &mut self.registers.hl.right,
            RegCode::HL => &mut self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Invalid RegCode for bit check")
        };

        if !to_set.nth_bit_as_bool(bit) {
            *to_set |= 2_u8.pow(bit as u32);
        }
    }

    pub fn bit_reset(&mut self, bit: usize, code: RegCode) {
        let to_set = match code {
            RegCode::A => &mut self.registers.af.left,
            RegCode::B => &mut self.registers.bc.left,
            RegCode::C => &mut self.registers.bc.right,
            RegCode::D => &mut self.registers.de.left,
            RegCode::E => &mut self.registers.de.right,
            RegCode::H => &mut self.registers.hl.left,
            RegCode::L => &mut self.registers.hl.right,
            RegCode::HL => &mut self.memory[self.registers.hl.take_as_one().into()],
            _ => panic!("Invalid RegCode for bit check")
        };

        if to_set.nth_bit_as_bool(bit) {
            *to_set ^= 2_u8.pow(bit as u32);
        }
    }

}
