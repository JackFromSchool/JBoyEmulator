pub mod memory;
pub mod register;

use self::memory::Memory;
use self::register::Registers;

enum RegCode {
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

struct Cpu {
    memory: Memory,
    registers: Registers,
}

impl Cpu {

    fn new() -> Self {
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
    fn load8(&mut self, target: RegCode, source: RegCode) {
        let val = match source {
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

    fn load16(&mut self, target: RegCode, source: RegCode) {
        let val = match source {
            RegCode::BC => self.registers.bc.take_as_one(),
            RegCode::DE => self.registers.de.take_as_one(),
            RegCode::HL => self.registers.hl.take_as_one(),
            RegCode::SP => self.registers.sp,
            RegCode::Const16(i) => i,
            _ => panic!("Invalid RegCode used as target for load16"),
        };

        let val = match source {
            RegCode::BC => self.registers.bc.change_as_one(val),
            RegCode::DE => self.registers.de.change_as_one(val),
            RegCode::HL => self.registers.hl.change_as_one(val),
            RegCode::SP => self.registers.sp = val,
            RegCode::Const16(_) => (),
            _ => panic!("Invalid RegCode used as target for load16"),
        };
    }

}
