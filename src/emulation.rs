use crate::cpu::RegCode;
use crate::cpu::CondCode;
use crate::cpu::Cpu;

enum Instruction {
    NOP,
    STOP,
    JR(CondCode),
    LD16(RegCode, RegCode),
    LD8(RegCode, RegCode),
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
}

fn fetch(cpu: &mut Cpu) -> Instruction {
    let nibble = cpu.current_pc_byte();

    let instruction = match nibble {
        0x00 => Instruction::NOP,
        0x10 => Instruction::STOP,
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
        0x22 => todo!(),
        0x32 => todo!(),
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

    };

    cpu.increment_pc();

    instruction
}
