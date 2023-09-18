pub mod memory;
pub mod register;

use self::memory::Memory;
use self::register::Registers;

enum RegCode {
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

struct Cpu {
    memory: Memory,
    registers: Registers,
}

impl Cpu {

    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
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

        let val = match target {
            RegCode::BC => self.registers.bc.change_as_one(val),
            RegCode::DE => self.registers.de.change_as_one(val),
            RegCode::HL => self.registers.hl.change_as_one(val),
            RegCode::SP => self.registers.sp = val,
            RegCode::Const16(i) => {
                self.memory[i as usize] = ((val & 0xFF00) >> 8) as u8;
                self.memory[i as usize] = (val & 0xFF) as u8;
            },
            _ => panic!("Invalid RegCode used as target for load16"),
        };
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
            RegCode::BC => self.registers.bc.change_as_one(self.registers.bc.take_as_one() + 1),
            RegCode::DE => self.registers.de.change_as_one(self.registers.de.take_as_one() + 1),
            RegCode::HL => self.registers.hl.change_as_one(self.registers.hl.take_as_one() + 1),
            RegCode::SP => self.registers.sp += 1,
            _ => panic!("Invalid RegCode used as target for increment16")
        }
    }
    
    /*
     *  DEC instruction for u8 and u16
     *  target register or the memory it points to are decremented by 1
     */
    pub fn decrement8(&mut self, target: RegCode) {
        match target {
            RegCode::A => self.registers.af.left -= 1,
            RegCode::B => self.registers.bc.left -= 1,
            RegCode::C => self.registers.bc.right -= 1,
            RegCode::D => self.registers.de.left -= 1,
            RegCode::E => self.registers.de.right -= 1,
            RegCode::H => self.registers.hl.left -= 1,
            RegCode::L => self.registers.hl.right -= 1,
            RegCode::HL => self.memory[self.registers.hl.take_as_one().into()] -= 1,
            _ => panic!("Invalid RegCode used as target for increment8"),
        }
    }

    pub fn decrement16(&mut self, target: RegCode) {
        match target {
            RegCode::BC => self.registers.bc.change_as_one(self.registers.bc.take_as_one() - 1),
            RegCode::DE => self.registers.de.change_as_one(self.registers.de.take_as_one() - 1),
            RegCode::HL => self.registers.hl.change_as_one(self.registers.hl.take_as_one() - 1),
            RegCode::SP => self.registers.sp -= 1,
            _ => panic!("Invalid RegCode used as target for increment16")
        }
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
        let _ = self.registers.af.left <= 1;
        if self.registers.af.is_carry_high() {
            self.registers.af.left += 1;
            self.registers.af.flip_carry_flag();
        }
        if carry_end_high && !self.registers.af.is_carry_high() {
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
        let _ = self.registers.af.left <= 1;
        if carry_end_high && !self.registers.af.is_carry_high() {
            self.registers.af.flip_carry_flag();
        }
        if carry_end_high {
            self.registers.af.left += 1;
        }
    }
    
    /*
     *  RRA instruction
     *  Rotates bits in register a right through the carry bit
     */
    pub fn rotate_right_carry_a(&mut self) {
        let carry_end_high = (self.registers.af.left | 1) == self.registers.af.left;
        let _ = self.registers.af.left >= 1;
        if self.registers.af.is_carry_high() {
            self.registers.af.left += 0b10000000;
            self.registers.af.flip_carry_flag();
        }
        if carry_end_high && !self.registers.af.is_carry_high() {
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
        let _ = self.registers.af.left >= 1;
        if carry_end_high && !self.registers.af.is_carry_high() {
            self.registers.af.flip_carry_flag();
        }
        if carry_end_high {
            self.registers.af.left += 0b10000000;
        }
    }
    
    /*
     *  ADD instruction for u8s
     *  Adds the register value to A
     */
    pub fn add8(&mut self, source: RegCode) {
        self.registers.af.left += match source {
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
        }
    }

    /*
     *  ADD instruction for u16s
     *  Adds the source value to target
     */

}
