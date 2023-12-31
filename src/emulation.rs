use crate::cpu::RegCode;
use crate::cpu::CondCode;
use crate::cpu::Cpu;

pub enum Instruction {
    PLACEHOLDER,
    NOP,
    STOP,
    HALT,
    DI,
    EI,
    JR(CondCode, i8),
    LD16(RegCode, RegCode),
    LD8(RegCode, RegCode),
    LDW(RegCode, RegCode),
    LDD(RegCode, RegCode),
    LDI(RegCode, RegCode),
    INC16(RegCode),
    INC8(RegCode),
    DEC16(RegCode),
    DEC8(RegCode),
    RLCA,
    RLA,
    RRCA,
    RRA,
    ADDSP(i8),
    ADD8(RegCode),
    ADD16(RegCode),
    SUB(RegCode),
    AND(RegCode),
    XOR(RegCode),
    OR(RegCode),
    SBC(RegCode),
    ADC(RegCode),
    CP(RegCode),
    PUSH(RegCode),
    POP(RegCode),
    RET(CondCode),
    RETI,
    JP(CondCode, u16),
    JPHL,
    RST(u16),
    CALL(CondCode, u16),
    RLC(RegCode),
    RRC(RegCode),
    RL(RegCode),
    RR(RegCode),
    SLA(RegCode),
    SRA(RegCode),
    SRL(RegCode),
    SWAP(RegCode),
    BIT(usize, RegCode),
    RES(usize, RegCode),
    SET(usize, RegCode),
}

impl std::fmt::Display for Instruction {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::PLACEHOLDER => {
                write!(f, "Placeholder Code")
            },
            Instruction::NOP => {
                write!(f, "NOP")
            },
            Instruction::STOP => {
                write!(f, "STOP")
            },
            Instruction::HALT => {
                write!(f, "HALT")
            },
            Instruction::DI => {
                write!(f, "Disable Interrupts")
            },
            Instruction::EI => {
                write!(f, "Enable Interrupts")
            },
            Instruction::JR(code, by) => {
                write!(f, "Relative jump based on {}, and by {:#04x}", code, by)
            },
            Instruction::LD16(target, source) => {
                write!(f, "LD from {}, into {}", source, target)
            },
            Instruction::LD8(target, source) => {
                write!(f, "LD")
            },
            Instruction::LDW(target, source) => {
                write!(f, "LD from {}, into {}", source, target)
            },
            Instruction::LDD(target, source) => {
                write!(f, "LD from {}, into {}", source, target)
            },
            Instruction::LDI(target, source) => {
                write!(f, "LD from {}, into {}", source, target)
            },
            Instruction::INC16(code) => {
                write!(f, "Incrementing {}", code)
            },
            Instruction::INC8(code) => {
                write!(f, "Incrementing {}", code)
            },
            Instruction::DEC16(code) => {
                write!(f, "Decrementing {}", code)
            },
            Instruction::DEC8(code) => {
                write!(f, "Decrementing {}", code)
            },
            Instruction::RLCA => {
                write!(f, "Rotating A left")
            },
            Instruction::RLA => {
                write!(f, "Rotating A left through carry")
            },
            Instruction::RRCA => {
                write!(f, "Rotating A right")
            },
            Instruction::RRA => {
                write!(f, "Rotating A right through carry")
            },
            Instruction::ADDSP(i) => {
                write!(f, "Adding {:#04x} to the stack pointer", i)
            },
            Instruction::ADD8(code) => {
                write!(f, "Adding to a from {}", code)
            },
            Instruction::ADD16(code) => {
                write!(f, "Adding to a from {}", code)
            },
            Instruction::SUB(code) => {
                write!(f, "Subtracting from a by {}", code)
            },
            Instruction::AND(code) => {
                write!(f, "Anding a by {}", code)
            },
            Instruction::XOR(code) => {
                write!(f, "XORing a by {}", code)
            },
            Instruction::OR(code) => {
                write!(f, "ORing a by {}", code)
            },
            Instruction::SBC(code) => {
                write!(f, "Subtracting to a with cary by {}", code)
            },
            Instruction::ADC(code) => {
                write!(f, "Adding to a with carry by {}", code)
            },
            Instruction::CP(code) => {
                write!(f, "Comparing by {}", code)
            },
            Instruction::PUSH(code) => {
                write!(f, "Pushing from {}", code)
            },
            Instruction::POP(code) => {
                write!(f, "Poping into {}", code)
            },
            Instruction::RET(code) => {
                write!(f, "Returning based on {}", code)
            },
            Instruction::RETI => {
                write!(f, "Returning and enabling Interrupts")
            },
            Instruction::JP(code, i) => {
                write!(f, "Jumping based on {}, to {:#06x}", code, i)
            },
            Instruction::JPHL => {
                write!(f, "Jumping to HL")
            },
            Instruction::RST(i) => {
                write!(f, "Calling RST and going to {:#06x}", i)
            },
            Instruction::CALL(code, i) => {
                write!(f, "Calling function at {:#06x}, based on {}", i, code)
            },
            Instruction::RLC(code) => {
                write!(f, "Rotating left at {}", code)
            },
            Instruction::RRC(code) => {
                write!(f, "Rotating right at {}", code)
            },
            Instruction::RL(code) => {
                write!(f, "Rotating left through the carry at {}", code)
            },
            Instruction::RR(code) => {
                write!(f, "Rotating right through the carry at {}", code)
            },
            Instruction::SLA(code) => {
                write!(f, "Shifting left at {}", code)
            },
            Instruction::SRA(code) => {
                write!(f, "Shifting right arithmetically at {}", code)
            },
            Instruction::SRL(code) => {
                write!(f, "Shifting right logically at {}", code)
            },
            Instruction::SWAP(code) => {
                write!(f, "Swapping bytes at {}", code)
            },
            Instruction::BIT(b, code) => {
                write!(f, "Checking bit {} at {}", b, code)
            },
            Instruction::RES(b, code) => {
                write!(f, "Unsetting bit {} at {}", b, code)
            },
            Instruction::SET(b, code) => {
                write!(f, "Setting bit {} at {}", b, code)
            },
        }
    }

}

pub fn fetch(cpu: &mut Cpu) -> Instruction {
    let nibble = cpu.current_pc_byte();
    
    /*
    clearscreen::clear().unwrap();
    println!("Registers:");
    println!("A:{: >4} F:{: >4}", cpu.registers.af.left, cpu.registers.af.right);
    println!("B:{: >4} C:{: >4}", cpu.registers.bc.left, cpu.registers.bc.right);
    println!("D:{: >4} E:{: >4}", cpu.registers.de.left, cpu.registers.de.right);
    println!("H:{: >4} L:{: >4}", cpu.registers.hl.left, cpu.registers.hl.right);
    println!("SP:{: >8}", cpu.registers.sp);
    println!("PC:{: >8}", cpu.registers.pc);
    println!("Opcode: {:#04x}", nibble);
    */

    let mut prefixed = false;

    let mut instruction = match nibble {
        
        // Instruction that doesn't exist but helps debug
        0xD3 => {
            println!("{}", cpu.registers.af.left);
            Instruction::NOP
        },

        0x00 => Instruction::NOP,
        0x10 => Instruction::STOP,
        0x20 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte() as i8;
            Instruction::JR(CondCode::NZ, val)
        },
        0x30 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte() as i8;
            Instruction::JR(CondCode::NC, val)
        }
        0x01 => {
            cpu.increment_pc();
            let code = RegCode::Const16(cpu.get_16_pc());
            Instruction::LD16(RegCode::BC, code)
        },
        0x11 => {
            cpu.increment_pc();
            let code = RegCode::Const16(cpu.get_16_pc());
            Instruction::LD16(RegCode::DE, code)
        },
        0x21 => {
            cpu.increment_pc();
            let code = RegCode::Const16(cpu.get_16_pc());
            Instruction::LD16(RegCode::HL, code)
        },
        0x31 => {
            cpu.increment_pc();
            let code = RegCode::Const16(cpu.get_16_pc());
            Instruction::LD16(RegCode::SP, code)
        },
        0x02 => Instruction::LD8(RegCode::BC, RegCode::A),
        0x12 => Instruction::LD8(RegCode::DE, RegCode::A),
        0x22 => Instruction::LDI(RegCode::HL, RegCode::A),
        0x32 => Instruction::LDD(RegCode::HL, RegCode::A),
        0x03 => Instruction::INC16(RegCode::BC),
        0x13 => Instruction::INC16(RegCode::DE),
        0x23 => Instruction::INC16(RegCode::HL),
        0x33 => Instruction::INC16(RegCode::SP),
        0x04 => Instruction::INC8(RegCode::B),
        0x14 => Instruction::INC8(RegCode::D),
        0x24 => Instruction::INC8(RegCode::H),
        0x34 => Instruction::INC8(RegCode::HL),
        0x05 => Instruction::DEC8(RegCode::B),
        0x15 => Instruction::DEC8(RegCode::D),
        0x25 => Instruction::DEC8(RegCode::H),
        0x35 => Instruction::DEC8(RegCode::HL),
        0x06 => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::B, code)
        },
        0x16 => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::D, code)
        },
        0x26 => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::H, code)
        },
        0x36 => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::HL, code)
        },
        0x07 => Instruction::RLCA,
        0x17 => Instruction::RLA,
        0x27 => todo!(),
        0x37 => todo!(),
        0x08 => {
            cpu.increment_pc();
            let code = RegCode::Const16(cpu.get_16_pc());
            Instruction::LD16(code, RegCode::SP)
        },
        0x18 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte() as i8;
            Instruction::JR(CondCode::Always, val)
        },
        0x28 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte() as i8;
            Instruction::JR(CondCode::Z, val)
        },
        0x38 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte() as i8;
            Instruction::JR(CondCode::C, val)
        },
        0x09 => Instruction::ADD16(RegCode::BC),
        0x19 => Instruction::ADD16(RegCode::DE),
        0x29 => Instruction::ADD16(RegCode::HL),
        0x39 => Instruction::ADD16(RegCode::SP),
        0x0A => Instruction::LD8(RegCode::A, RegCode::BC),
        0x1A => Instruction::LD8(RegCode::A, RegCode::DE),
        0x2A => Instruction::LDI(RegCode::A, RegCode::HL),
        0x3A => Instruction::LDD(RegCode::A, RegCode::HL),
        0x0B => Instruction::DEC16(RegCode::BC),
        0x1B => Instruction::DEC16(RegCode::DE),
        0x2B => Instruction::DEC16(RegCode::HL),
        0x3B => Instruction::DEC16(RegCode::SP),
        0x0C => Instruction::INC8(RegCode::C),
        0x1C => Instruction::INC8(RegCode::E),
        0x2C => Instruction::INC8(RegCode::L),
        0x3C => Instruction::INC8(RegCode::A),
        0x0D => Instruction::DEC8(RegCode::C),
        0x1D => Instruction::DEC8(RegCode::E),
        0x2D => Instruction::DEC8(RegCode::L),
        0x3D => Instruction::DEC8(RegCode::A),
        0x0E => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::C, code)
        },
        0x1E => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::E, code)
        },
        0x2E => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::L, code)
        },
        0x3E => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LD8(RegCode::A, code)
        },
        0x0F => Instruction::RRCA,
        0x1F => Instruction::RRA,
        0x2F => todo!(),
        0x3F => todo!(),
        0x40 => Instruction::LD8(RegCode::B, RegCode::B),
        0x41 => Instruction::LD8(RegCode::B, RegCode::C),
        0x42 => Instruction::LD8(RegCode::B, RegCode::D),
        0x43 => Instruction::LD8(RegCode::B, RegCode::E),
        0x44 => Instruction::LD8(RegCode::B, RegCode::H),
        0x45 => Instruction::LD8(RegCode::B, RegCode::L),
        0x46 => Instruction::LD8(RegCode::B, RegCode::HL),
        0x47 => Instruction::LD8(RegCode::B, RegCode::A),
        0x48 => Instruction::LD8(RegCode::C, RegCode::B),
        0x49 => Instruction::LD8(RegCode::C, RegCode::C),
        0x4A => Instruction::LD8(RegCode::C, RegCode::D),
        0x4B => Instruction::LD8(RegCode::C, RegCode::E),
        0x4C => Instruction::LD8(RegCode::C, RegCode::H),
        0x4D => Instruction::LD8(RegCode::C, RegCode::L),
        0x4E => Instruction::LD8(RegCode::C, RegCode::HL),
        0x4F => Instruction::LD8(RegCode::C, RegCode::A),
        0x50 => Instruction::LD8(RegCode::D, RegCode::B),
        0x51 => Instruction::LD8(RegCode::D, RegCode::C),
        0x52 => Instruction::LD8(RegCode::D, RegCode::D),
        0x53 => Instruction::LD8(RegCode::D, RegCode::E),
        0x54 => Instruction::LD8(RegCode::D, RegCode::H),
        0x55 => Instruction::LD8(RegCode::D, RegCode::L),
        0x56 => Instruction::LD8(RegCode::D, RegCode::HL),
        0x57 => Instruction::LD8(RegCode::D, RegCode::A),
        0x58 => Instruction::LD8(RegCode::E, RegCode::B),
        0x59 => Instruction::LD8(RegCode::E, RegCode::C),
        0x5A => Instruction::LD8(RegCode::E, RegCode::D),
        0x5B => Instruction::LD8(RegCode::E, RegCode::E),
        0x5C => Instruction::LD8(RegCode::E, RegCode::H),
        0x5D => Instruction::LD8(RegCode::E, RegCode::L),
        0x5E => Instruction::LD8(RegCode::E, RegCode::HL),
        0x5F => Instruction::LD8(RegCode::E, RegCode::A),
        0x60 => Instruction::LD8(RegCode::H, RegCode::B),
        0x61 => Instruction::LD8(RegCode::H, RegCode::C),
        0x62 => Instruction::LD8(RegCode::H, RegCode::D),
        0x63 => Instruction::LD8(RegCode::H, RegCode::E),
        0x64 => Instruction::LD8(RegCode::H, RegCode::H),
        0x65 => Instruction::LD8(RegCode::H, RegCode::L),
        0x66 => Instruction::LD8(RegCode::H, RegCode::HL),
        0x67 => Instruction::LD8(RegCode::H, RegCode::A),
        0x68 => Instruction::LD8(RegCode::L, RegCode::B),
        0x69 => Instruction::LD8(RegCode::L, RegCode::C),
        0x6A => Instruction::LD8(RegCode::L, RegCode::D),
        0x6B => Instruction::LD8(RegCode::L, RegCode::E),
        0x6C => Instruction::LD8(RegCode::L, RegCode::H),
        0x6D => Instruction::LD8(RegCode::L, RegCode::L),
        0x6E => Instruction::LD8(RegCode::L, RegCode::HL),
        0x6F => Instruction::LD8(RegCode::L, RegCode::A),
        0x70 => Instruction::LD8(RegCode::HL, RegCode::B),
        0x71 => Instruction::LD8(RegCode::HL, RegCode::C),
        0x72 => Instruction::LD8(RegCode::HL, RegCode::D),
        0x73 => Instruction::LD8(RegCode::HL, RegCode::E),
        0x74 => Instruction::LD8(RegCode::HL, RegCode::H),
        0x75 => Instruction::LD8(RegCode::HL, RegCode::L),
        0x76 => Instruction::HALT,
        0x77 => Instruction::LD8(RegCode::HL, RegCode::A),
        0x78 => Instruction::LD8(RegCode::A, RegCode::B),
        0x79 => Instruction::LD8(RegCode::A, RegCode::C),
        0x7A => Instruction::LD8(RegCode::A, RegCode::D),
        0x7B => Instruction::LD8(RegCode::A, RegCode::E),
        0x7C => Instruction::LD8(RegCode::A, RegCode::H),
        0x7D => Instruction::LD8(RegCode::A, RegCode::L),
        0x7E => Instruction::LD8(RegCode::A, RegCode::HL),
        0x7F => Instruction::LD8(RegCode::A, RegCode::A),
        0x80 => Instruction::ADD8(RegCode::B),
        0x81 => Instruction::ADD8(RegCode::C),
        0x82 => Instruction::ADD8(RegCode::D),
        0x83 => Instruction::ADD8(RegCode::E),
        0x84 => Instruction::ADD8(RegCode::H),
        0x85 => Instruction::ADD8(RegCode::L),
        0x86 => Instruction::ADD8(RegCode::HL),
        0x87 => Instruction::ADD8(RegCode::A),
        0x88 => Instruction::ADC(RegCode::B),
        0x89 => Instruction::ADC(RegCode::C),
        0x8A => Instruction::ADC(RegCode::D),
        0x8B => Instruction::ADC(RegCode::E),
        0x8C => Instruction::ADC(RegCode::H),
        0x8D => Instruction::ADC(RegCode::L),
        0x8E => Instruction::ADC(RegCode::HL),
        0x8F => Instruction::ADC(RegCode::A),
        0x90 => Instruction::SUB(RegCode::B),
        0x91 => Instruction::SUB(RegCode::C),
        0x92 => Instruction::SUB(RegCode::D),
        0x93 => Instruction::SUB(RegCode::E),
        0x94 => Instruction::SUB(RegCode::H),
        0x95 => Instruction::SUB(RegCode::L),
        0x96 => Instruction::SUB(RegCode::HL),
        0x97 => Instruction::SUB(RegCode::A),
        0x98 => Instruction::SBC(RegCode::B),
        0x99 => Instruction::SBC(RegCode::C),
        0x9A => Instruction::SBC(RegCode::D),
        0x9B => Instruction::SBC(RegCode::E),
        0x9C => Instruction::SBC(RegCode::H),
        0x9D => Instruction::SBC(RegCode::L),
        0x9E => Instruction::SBC(RegCode::HL),
        0x9F => Instruction::SBC(RegCode::A),
        0xA0 => Instruction::AND(RegCode::B),
        0xA1 => Instruction::AND(RegCode::C),
        0xA2 => Instruction::AND(RegCode::D),
        0xA3 => Instruction::AND(RegCode::E),
        0xA4 => Instruction::AND(RegCode::H),
        0xA5 => Instruction::AND(RegCode::L),
        0xA6 => Instruction::AND(RegCode::HL),
        0xA7 => Instruction::AND(RegCode::A),
        0xA8 => Instruction::XOR(RegCode::B),
        0xA9 => Instruction::XOR(RegCode::C),
        0xAA => Instruction::XOR(RegCode::D),
        0xAB => Instruction::XOR(RegCode::E),
        0xAC => Instruction::XOR(RegCode::H),
        0xAD => Instruction::XOR(RegCode::L),
        0xAE => Instruction::XOR(RegCode::HL),
        0xAF => Instruction::XOR(RegCode::A),
        0xB0 => Instruction::OR(RegCode::B),
        0xB1 => Instruction::OR(RegCode::C),
        0xB2 => Instruction::OR(RegCode::D),
        0xB3 => Instruction::OR(RegCode::E),
        0xB4 => Instruction::OR(RegCode::H),
        0xB5 => Instruction::OR(RegCode::L),
        0xB6 => Instruction::OR(RegCode::HL),
        0xB7 => Instruction::OR(RegCode::A),
        0xB8 => Instruction::CP(RegCode::B),
        0xB9 => Instruction::CP(RegCode::C),
        0xBA => Instruction::CP(RegCode::D),
        0xBB => Instruction::CP(RegCode::E),
        0xBC => Instruction::CP(RegCode::H),
        0xBD => Instruction::CP(RegCode::L),
        0xBE => Instruction::CP(RegCode::HL),
        0xBF => Instruction::CP(RegCode::A),
        0xC0 => Instruction::RET(CondCode::NZ),
        0xD0 => Instruction::RET(CondCode::NC),
        0xE0 => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LDW(code, RegCode::A)
        },
        0xF0 => {
            cpu.increment_pc();
            let code = RegCode::Const8(cpu.current_pc_byte());
            Instruction::LDW(RegCode::A, code)
        },
        0xC1 => Instruction::POP(RegCode::BC),
        0xD1 => Instruction::POP(RegCode::DE),
        0xE1 => Instruction::POP(RegCode::HL),
        0xF1 => Instruction::POP(RegCode::AF),
        0xC2 => {
            cpu.increment_pc();
            let jump_to = cpu.get_16_pc();
            Instruction::JP(CondCode::NZ, jump_to)
        },
        0xD2 => {
            cpu.increment_pc();
            let jump_to = cpu.get_16_pc();
            Instruction::JP(CondCode::NC, jump_to)
        },
        0xE2 => Instruction::LDW(RegCode::C, RegCode::A),
        0xF2 => Instruction::LDW(RegCode::A, RegCode::C),
        0xC3 => {
            cpu.increment_pc();
            let jump_to = cpu.get_16_pc();
            Instruction::JP(CondCode::Always, jump_to)
        },
        0xF3 => Instruction::DI,
        0xC4 => {
            cpu.increment_pc();
            let function_at = cpu.get_16_pc();
            Instruction::CALL(CondCode::NZ, function_at)
        }
        0xD4 => {
            cpu.increment_pc();
            let function_at = cpu.get_16_pc();
            Instruction::CALL(CondCode::NC, function_at)
        },
        0xC5 => Instruction::PUSH(RegCode::BC),
        0xD5 => Instruction::PUSH(RegCode::DE),
        0xE5 => Instruction::PUSH(RegCode::HL),
        0xF5 => Instruction::PUSH(RegCode::AF),
        0xC6 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::ADD8(RegCode::Const8(val))
        }
        0xD6 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::SUB(RegCode::Const8(val))
        }
        0xE6 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::AND(RegCode::Const8(val))
        }
        0xF6 => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::OR(RegCode::Const8(val))
        },
        0xC7 => Instruction::RST(0x00),
        0xD7 => Instruction::RST(0x10),
        0xE7 => Instruction::RST(0x20),
        0xF7 => Instruction::RST(0x30),
        0xC8 => Instruction::RET(CondCode::Z),
        0xD8 => Instruction::RET(CondCode::C),
        0xE8 => todo!(),
        0xF8 => todo!(),
        0xC9 => Instruction::RET(CondCode::Always),
        0xD9 => Instruction::RETI,
        0xE9 => Instruction::JPHL,
        0xF9 => todo!(),
        0xCA => {
            cpu.increment_pc();
            let jump_to = cpu.get_16_pc();
            Instruction::JP(CondCode::Z, jump_to)
        },
        0xDA => {
            cpu.increment_pc();
            let jump_to = cpu.get_16_pc();
            Instruction::JP(CondCode::C, jump_to)
        },
        0xEA => {
            cpu.increment_pc();
            let code = RegCode::Const16(cpu.get_16_pc());
            Instruction::LDW(code, RegCode::A)
        }
        0xFA => {
            cpu.increment_pc();
            let code = RegCode::Const16(cpu.get_16_pc());
            Instruction::LDW(RegCode::A, code)
        },
        0xCB => {
            prefixed = true;
            Instruction::PLACEHOLDER
        },
        0xFB => Instruction::EI,
        0xCC => {
            cpu.increment_pc();
            let function_at = cpu.get_16_pc();
            Instruction::CALL(CondCode::Z, function_at)
        },
        0xDC => {
            cpu.increment_pc();
            let function_at = cpu.get_16_pc();
            Instruction::CALL(CondCode::C, function_at)
        },
        0xCD => {
            cpu.increment_pc();
            let function_at = cpu.get_16_pc();
            Instruction::CALL(CondCode::Always, function_at)
        },
        0xCE => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::ADC(RegCode::Const8(val))
        },
        0xDE => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::SBC(RegCode::Const8(val))
        },
        0xEE => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::XOR(RegCode::Const8(val))
        },
        0xFE => {
            cpu.increment_pc();
            let val = cpu.current_pc_byte();
            Instruction::CP(RegCode::Const8(val))
        },
        0xCF => Instruction::RST(0x08),
        0xDF => Instruction::RST(0x18),
        0xEF => Instruction::RST(0x28),
        0xFF => Instruction::RST(0x38),
        
        _ => panic!("Invalid Opcode: {nibble} | Instruction could not be fetched.")
    };

    cpu.increment_pc();

    if prefixed {
        instruction = match cpu.current_pc_byte() {
            0x0 => Instruction::RLC(RegCode::B),
            0x1 => Instruction::RLC(RegCode::C),
            0x2 => Instruction::RLC(RegCode::D),
            0x3 => Instruction::RLC(RegCode::E),
            0x4 => Instruction::RLC(RegCode::H),
            0x5 => Instruction::RLC(RegCode::L),
            0x6 => Instruction::RLC(RegCode::HL),
            0x7 => Instruction::RLC(RegCode::A),
            0x8 => Instruction::RRC(RegCode::B),
            0x9 => Instruction::RRC(RegCode::C),
            0xa => Instruction::RRC(RegCode::D),
            0xb => Instruction::RRC(RegCode::E),
            0xc => Instruction::RRC(RegCode::H),
            0xd => Instruction::RRC(RegCode::L),
            0xe => Instruction::RRC(RegCode::HL),
            0xf => Instruction::RRC(RegCode::A),
            0x10 => Instruction::RL(RegCode::B),
            0x11 => Instruction::RL(RegCode::C),
            0x12 => Instruction::RL(RegCode::D),
            0x13 => Instruction::RL(RegCode::E),
            0x14 => Instruction::RL(RegCode::H),
            0x15 => Instruction::RL(RegCode::L),
            0x16 => Instruction::RL(RegCode::HL),
            0x17 => Instruction::RL(RegCode::A),
            0x18 => Instruction::RR(RegCode::B),
            0x19 => Instruction::RR(RegCode::C),
            0x1a => Instruction::RR(RegCode::D),
            0x1b => Instruction::RR(RegCode::E),
            0x1c => Instruction::RR(RegCode::H),
            0x1d => Instruction::RR(RegCode::L),
            0x1e => Instruction::RR(RegCode::HL),
            0x1f => Instruction::RR(RegCode::A),
            0x20 => Instruction::SLA(RegCode::B),
            0x21 => Instruction::SLA(RegCode::C),
            0x22 => Instruction::SLA(RegCode::D),
            0x23 => Instruction::SLA(RegCode::E),
            0x24 => Instruction::SLA(RegCode::H),
            0x25 => Instruction::SLA(RegCode::L),
            0x26 => Instruction::SLA(RegCode::HL),
            0x27 => Instruction::SLA(RegCode::A),
            0x28 => Instruction::SRA(RegCode::B),
            0x29 => Instruction::SRA(RegCode::C),
            0x2a => Instruction::SRA(RegCode::D),
            0x2b => Instruction::SRA(RegCode::E),
            0x2c => Instruction::SRA(RegCode::H),
            0x2d => Instruction::SRA(RegCode::L),
            0x2e => Instruction::SRA(RegCode::HL),
            0x2f => Instruction::SRA(RegCode::A),
            0x30 => Instruction::SWAP(RegCode::B),
            0x31 => Instruction::SWAP(RegCode::C),
            0x32 => Instruction::SWAP(RegCode::D),
            0x33 => Instruction::SWAP(RegCode::E),
            0x34 => Instruction::SWAP(RegCode::H),
            0x35 => Instruction::SWAP(RegCode::L),
            0x36 => Instruction::SWAP(RegCode::HL),
            0x37 => Instruction::SWAP(RegCode::A),
            0x38 => Instruction::SRL(RegCode::B),
            0x39 => Instruction::SRL(RegCode::C),
            0x3a => Instruction::SRL(RegCode::D),
            0x3b => Instruction::SRL(RegCode::E),
            0x3c => Instruction::SRL(RegCode::H),
            0x3d => Instruction::SRL(RegCode::L),
            0x3e => Instruction::SRL(RegCode::HL),
            0x3f => Instruction::SRL(RegCode::A),
            0x40 => Instruction::BIT(0, RegCode::B),
            0x41 => Instruction::BIT(0, RegCode::C),
            0x42 => Instruction::BIT(0, RegCode::D),
            0x43 => Instruction::BIT(0, RegCode::E),
            0x44 => Instruction::BIT(0, RegCode::H),
            0x45 => Instruction::BIT(0, RegCode::L),
            0x46 => Instruction::BIT(0, RegCode::HL),
            0x47 => Instruction::BIT(0, RegCode::A),
            0x48 => Instruction::BIT(1, RegCode::B),
            0x49 => Instruction::BIT(1, RegCode::C),
            0x4a => Instruction::BIT(1, RegCode::D),
            0x4b => Instruction::BIT(1, RegCode::E),
            0x4c => Instruction::BIT(1, RegCode::H),
            0x4d => Instruction::BIT(1, RegCode::L),
            0x4e => Instruction::BIT(1, RegCode::HL),
            0x4f => Instruction::BIT(1, RegCode::A),
            0x50 => Instruction::BIT(2, RegCode::B),
            0x51 => Instruction::BIT(2, RegCode::C),
            0x52 => Instruction::BIT(2, RegCode::D),
            0x53 => Instruction::BIT(2, RegCode::E),
            0x54 => Instruction::BIT(2, RegCode::H),
            0x55 => Instruction::BIT(2, RegCode::L),
            0x56 => Instruction::BIT(2, RegCode::HL),
            0x57 => Instruction::BIT(2, RegCode::A),
            0x58 => Instruction::BIT(3, RegCode::B),
            0x59 => Instruction::BIT(3, RegCode::C),
            0x5a => Instruction::BIT(3, RegCode::D),
            0x5b => Instruction::BIT(3, RegCode::E),
            0x5c => Instruction::BIT(3, RegCode::H),
            0x5d => Instruction::BIT(3, RegCode::L),
            0x5e => Instruction::BIT(3, RegCode::HL),
            0x5f => Instruction::BIT(3, RegCode::A),
            0x60 => Instruction::BIT(4, RegCode::B),
            0x61 => Instruction::BIT(4, RegCode::C),
            0x62 => Instruction::BIT(4, RegCode::D),
            0x63 => Instruction::BIT(4, RegCode::E),
            0x64 => Instruction::BIT(4, RegCode::H),
            0x65 => Instruction::BIT(4, RegCode::L),
            0x66 => Instruction::BIT(4, RegCode::HL),
            0x67 => Instruction::BIT(4, RegCode::A),
            0x68 => Instruction::BIT(5, RegCode::B),
            0x69 => Instruction::BIT(5, RegCode::C),
            0x6a => Instruction::BIT(5, RegCode::D),
            0x6b => Instruction::BIT(5, RegCode::E),
            0x6c => Instruction::BIT(5, RegCode::H),
            0x6d => Instruction::BIT(5, RegCode::L),
            0x6e => Instruction::BIT(5, RegCode::HL),
            0x6f => Instruction::BIT(5, RegCode::A),
            0x70 => Instruction::BIT(6, RegCode::B),
            0x71 => Instruction::BIT(6, RegCode::C),
            0x72 => Instruction::BIT(6, RegCode::D),
            0x73 => Instruction::BIT(6, RegCode::E),
            0x74 => Instruction::BIT(6, RegCode::H),
            0x75 => Instruction::BIT(6, RegCode::L),
            0x76 => Instruction::BIT(6, RegCode::HL),
            0x77 => Instruction::BIT(6, RegCode::A),
            0x78 => Instruction::BIT(7, RegCode::B),
            0x79 => Instruction::BIT(7, RegCode::C),
            0x7a => Instruction::BIT(7, RegCode::D),
            0x7b => Instruction::BIT(7, RegCode::E),
            0x7c => Instruction::BIT(7, RegCode::H),
            0x7d => Instruction::BIT(7, RegCode::L),
            0x7e => Instruction::BIT(7, RegCode::HL),
            0x7f => Instruction::BIT(7, RegCode::A),
            0x80 => Instruction::RES(0, RegCode::B),
            0x81 => Instruction::RES(0, RegCode::C),
            0x82 => Instruction::RES(0, RegCode::D),
            0x83 => Instruction::RES(0, RegCode::E),
            0x84 => Instruction::RES(0, RegCode::H),
            0x85 => Instruction::RES(0, RegCode::L),
            0x86 => Instruction::RES(0, RegCode::HL),
            0x87 => Instruction::RES(0, RegCode::A),
            0x88 => Instruction::RES(1, RegCode::B),
            0x89 => Instruction::RES(1, RegCode::C),
            0x8a => Instruction::RES(1, RegCode::D),
            0x8b => Instruction::RES(1, RegCode::E),
            0x8c => Instruction::RES(1, RegCode::H),
            0x8d => Instruction::RES(1, RegCode::L),
            0x8e => Instruction::RES(1, RegCode::HL),
            0x8f => Instruction::RES(1, RegCode::A),
            0x90 => Instruction::RES(2, RegCode::B),
            0x91 => Instruction::RES(2, RegCode::C),
            0x92 => Instruction::RES(2, RegCode::D),
            0x93 => Instruction::RES(2, RegCode::E),
            0x94 => Instruction::RES(2, RegCode::H),
            0x95 => Instruction::RES(2, RegCode::L),
            0x96 => Instruction::RES(2, RegCode::HL),
            0x97 => Instruction::RES(2, RegCode::A),
            0x98 => Instruction::RES(3, RegCode::B),
            0x99 => Instruction::RES(3, RegCode::C),
            0x9a => Instruction::RES(3, RegCode::D),
            0x9b => Instruction::RES(3, RegCode::E),
            0x9c => Instruction::RES(3, RegCode::H),
            0x9d => Instruction::RES(3, RegCode::L),
            0x9e => Instruction::RES(3, RegCode::HL),
            0x9f => Instruction::RES(3, RegCode::A),
            0xa0 => Instruction::RES(4, RegCode::B),
            0xa1 => Instruction::RES(4, RegCode::C),
            0xa2 => Instruction::RES(4, RegCode::D),
            0xa3 => Instruction::RES(4, RegCode::E),
            0xa4 => Instruction::RES(4, RegCode::H),
            0xa5 => Instruction::RES(4, RegCode::L),
            0xa6 => Instruction::RES(4, RegCode::HL),
            0xa7 => Instruction::RES(4, RegCode::A),
            0xa8 => Instruction::RES(5, RegCode::B),
            0xa9 => Instruction::RES(5, RegCode::C),
            0xaa => Instruction::RES(5, RegCode::D),
            0xab => Instruction::RES(5, RegCode::E),
            0xac => Instruction::RES(5, RegCode::H),
            0xad => Instruction::RES(5, RegCode::L),
            0xae => Instruction::RES(5, RegCode::HL),
            0xaf => Instruction::RES(5, RegCode::A),
            0xb0 => Instruction::RES(6, RegCode::B),
            0xb1 => Instruction::RES(6, RegCode::C),
            0xb2 => Instruction::RES(6, RegCode::D),
            0xb3 => Instruction::RES(6, RegCode::E),
            0xb4 => Instruction::RES(6, RegCode::H),
            0xb5 => Instruction::RES(6, RegCode::L),
            0xb6 => Instruction::RES(6, RegCode::HL),
            0xb7 => Instruction::RES(6, RegCode::A),
            0xb8 => Instruction::RES(7, RegCode::B),
            0xb9 => Instruction::RES(7, RegCode::C),
            0xba => Instruction::RES(7, RegCode::D),
            0xbb => Instruction::RES(7, RegCode::E),
            0xbc => Instruction::RES(7, RegCode::H),
            0xbd => Instruction::RES(7, RegCode::L),
            0xbe => Instruction::RES(7, RegCode::HL),
            0xbf => Instruction::RES(7, RegCode::A),
            0xc0 => Instruction::SET(0, RegCode::B),
            0xc1 => Instruction::SET(0, RegCode::C),
            0xc2 => Instruction::SET(0, RegCode::D),
            0xc3 => Instruction::SET(0, RegCode::E),
            0xc4 => Instruction::SET(0, RegCode::H),
            0xc5 => Instruction::SET(0, RegCode::L),
            0xc6 => Instruction::SET(0, RegCode::HL),
            0xc7 => Instruction::SET(0, RegCode::A),
            0xc8 => Instruction::SET(1, RegCode::B),
            0xc9 => Instruction::SET(1, RegCode::C),
            0xca => Instruction::SET(1, RegCode::D),
            0xcb => Instruction::SET(1, RegCode::E),
            0xcc => Instruction::SET(1, RegCode::H),
            0xcd => Instruction::SET(1, RegCode::L),
            0xce => Instruction::SET(1, RegCode::HL),
            0xcf => Instruction::SET(1, RegCode::A),
            0xd0 => Instruction::SET(2, RegCode::B),
            0xd1 => Instruction::SET(2, RegCode::C),
            0xd2 => Instruction::SET(2, RegCode::D),
            0xd3 => Instruction::SET(2, RegCode::E),
            0xd4 => Instruction::SET(2, RegCode::H),
            0xd5 => Instruction::SET(2, RegCode::L),
            0xd6 => Instruction::SET(2, RegCode::HL),
            0xd7 => Instruction::SET(2, RegCode::A),
            0xd8 => Instruction::SET(3, RegCode::B),
            0xd9 => Instruction::SET(3, RegCode::C),
            0xda => Instruction::SET(3, RegCode::D),
            0xdb => Instruction::SET(3, RegCode::E),
            0xdc => Instruction::SET(3, RegCode::H),
            0xdd => Instruction::SET(3, RegCode::L),
            0xde => Instruction::SET(3, RegCode::HL),
            0xdf => Instruction::SET(3, RegCode::A),
            0xe0 => Instruction::SET(4, RegCode::B),
            0xe1 => Instruction::SET(4, RegCode::C),
            0xe2 => Instruction::SET(4, RegCode::D),
            0xe3 => Instruction::SET(4, RegCode::E),
            0xe4 => Instruction::SET(4, RegCode::H),
            0xe5 => Instruction::SET(4, RegCode::L),
            0xe6 => Instruction::SET(4, RegCode::HL),
            0xe7 => Instruction::SET(4, RegCode::A),
            0xe8 => Instruction::SET(5, RegCode::B),
            0xe9 => Instruction::SET(5, RegCode::C),
            0xea => Instruction::SET(5, RegCode::D),
            0xeb => Instruction::SET(5, RegCode::E),
            0xec => Instruction::SET(5, RegCode::H),
            0xed => Instruction::SET(5, RegCode::L),
            0xee => Instruction::SET(5, RegCode::HL),
            0xef => Instruction::SET(5, RegCode::A),
            0xf0 => Instruction::SET(6, RegCode::B),
            0xf1 => Instruction::SET(6, RegCode::C),
            0xf2 => Instruction::SET(6, RegCode::D),
            0xf3 => Instruction::SET(6, RegCode::E),
            0xf4 => Instruction::SET(6, RegCode::H),
            0xf5 => Instruction::SET(6, RegCode::L),
            0xf6 => Instruction::SET(6, RegCode::HL),
            0xf7 => Instruction::SET(6, RegCode::A),
            0xf8 => Instruction::SET(7, RegCode::B),
            0xf9 => Instruction::SET(7, RegCode::C),
            0xfa => Instruction::SET(7, RegCode::D),
            0xfb => Instruction::SET(7, RegCode::E),
            0xfc => Instruction::SET(7, RegCode::H),
            0xfd => Instruction::SET(7, RegCode::L),
            0xfe => Instruction::SET(7, RegCode::HL),
            0xff => Instruction::SET(7, RegCode::A),
        }
    }

    instruction
}

pub fn run(cpu: &mut Cpu, instruction: Instruction) {
    match instruction {
        Instruction::STOP => {
            std::process::exit(0);
        },
        Instruction::NOP => {},
        Instruction::HALT => todo!(),
        Instruction::INC8(target) => {
            cpu.increment8(target);
        },
        Instruction::INC16(target) => {
            cpu.increment16(target)
        },
        Instruction::DEC8(target) => {
            cpu.decrement8(target);
        },
        Instruction::DEC16(target) => {
            cpu.decrement16(target);
        }
        Instruction::OR(target) => {
            cpu.or(target);
        },
        Instruction::AND(target) => {
            cpu.and(target);
        },
        Instruction::XOR(target) => {
            cpu.xor(target);
        },
        Instruction::ADD16(source) => {
            cpu.add16(source);
        },
        Instruction::ADD8(target) => {
            cpu.add8(target);
        },
        Instruction::ADDSP(val) => {
            cpu.add_sp(val);
        },
        Instruction::LD8(target, source) => {
            cpu.load8(target, source);
        },
        Instruction::LD16(target, source) => {
            cpu.load16(target, source);
        },
        Instruction::LDD(target, source) => {
            cpu.load_dec(target, source);
        },
        Instruction::LDI(target, source) => {
            cpu.load_inc(target, source);
        }
        Instruction::SUB(target) => {
            cpu.sub(target);
        },
        Instruction::ADC(target) => {
            cpu.addc(target);
        },
        Instruction::SBC(target) => {
            cpu.subc(target);
        },
        Instruction::RRCA => {
            cpu.rotate_right_a();
        }
        Instruction::RRA => {
            cpu.rotate_right_carry_a();
        },
        Instruction::RLCA => {
            cpu.rotate_left_a();
        },
        Instruction::RLA => {
            cpu.rotate_left_carry_a();
        },
        Instruction::JR(condition, val) => {
            println!("JR: {val}");
            cpu.jump_relative(condition, val);
        },
        Instruction::CP(target) => {
            cpu.cp(target);
        },
        Instruction::PUSH(target) => {
            cpu.push(target);
        },
        Instruction::POP(target) => {
            cpu.pop(target);
        },
        Instruction::RST(i) => {
            cpu.restart(i);
        },
        Instruction::CALL(condition, i) => {
            cpu.call(condition, i);
        },
        Instruction::RET(condition) => {
            cpu.ret(condition);
        },
        Instruction::LDW(target, source) => {
            cpu.load_weird(target, source);
        },
        Instruction::DI => {
            cpu.di();
        },
        Instruction::EI => {
            cpu.ei();
        },
        Instruction::JP(condition, i) => {
            cpu.jump(condition, i);
        },
        Instruction::RETI => {
            todo!()
        },
        Instruction::JPHL => {
            cpu.jump_hl();
        },
        Instruction::RLC(code) => {
            cpu.rotate_left_carry(code);
        },
        Instruction::RL(code) => {
            cpu.rotate_left(code);
        },
        Instruction::RRC(code) => {
            cpu.rotate_right_carry(code);
        },
        Instruction::RR(code) => {
            cpu.rotate_right(code);
        },
        Instruction::SLA(code) => {
            cpu.shift_left(code);
        },
        Instruction::SRA(code) => {
            cpu.shift_right_arithmetic(code);
        },
        Instruction::SRL(code) => {
            cpu.shift_right_logical(code);
        },
        Instruction::SWAP(code) => {
            cpu.swap(code);
        },
        Instruction::BIT(position, code) => {
            cpu.bit_check_zero(position, code);
        },
        Instruction::RES(position, code) => {
            cpu.bit_reset(position, code);
        },
        Instruction::SET(position, code) => {
            cpu.bit_set(position, code);
        }
        Instruction::PLACEHOLDER => {
            panic!("Placeholder instruction ran.")
        }
    }
}
