mod sdl;
mod cpu;
mod util;
mod emulation;
mod graphics;

use sdl::events::GBButton;
use self::cpu::Cpu;
use self::emulation::{fetch, run};
use std::io::{BufReader, Read};
use std::fs::File;

fn main() {
    let file = File::open("roms/logo.gb").unwrap();
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();

    let _ = reader.read_to_end(&mut bytes);

    let mut handles = sdl::SdlHandles::new();
    let mut cpu = Cpu::new_with_rom(&bytes);
    
    loop {
        handles.events.update_events();
        handles.canvas.update();
        
        let instruction = fetch(&mut cpu);

        println!("{}", instruction);
        loop {
            break;
            handles.events.update_events();
            if handles.events.is_pressed(crate::GBButton::A) {
                break;
            }
        }

        run(&mut cpu, instruction);
    }
}



#[cfg(test)]
mod tests {
    
    use crate::cpu::{ Cpu, CondCode, RegCode };
    
    #[test]
    fn load8_test() {
        let mut cpu = Cpu::new();
        
        cpu.load8(RegCode::A, RegCode::Const8(8));
        assert_eq!(cpu.registers.af.left, 8);

        cpu.registers.bc.change_as_one(23);
        cpu.load8(RegCode::BC, RegCode::A);
        assert_eq!(cpu.memory[23], 8);
    }
    
    #[test]
    fn load16_test() {
        let mut cpu = Cpu::new();
        
        cpu.load16(RegCode::BC, RegCode::Const16(8));
        assert_eq!(cpu.registers.bc.take_as_one(), 8);

        println!("{}", (17 & 0xFF00) >> 4);

        cpu.load16(RegCode::Const16(8), RegCode::Const16(0b00010001));
        assert_eq!(cpu.memory[8], 1);
        assert_eq!(cpu.memory[9], 1);
    }

    #[test]
    fn increment8_test() {
        let mut cpu = Cpu::new();

        cpu.increment8(RegCode::A);
        assert_eq!(cpu.registers.af.left, 1);

        cpu.increment8(RegCode::HL);
        assert_eq!(cpu.memory[0], 1);
    }

    #[test]
    fn increment16_test() {
        let mut cpu = Cpu::new();

        cpu.increment16(RegCode::BC);
        assert_eq!(cpu.registers.bc.take_as_one(), 1);
        
        cpu.increment16(RegCode::SP);
        assert_eq!(cpu.registers.sp, 0);
    }

    #[test]
    fn decrement8_test() {
        let mut cpu = Cpu::new();

        cpu.decrement8(RegCode::A);
        assert_eq!(cpu.registers.af.left, u8::MAX);

        cpu.decrement8(RegCode::HL);
        assert_eq!(cpu.memory[0], 255);
    }

    #[test]
    fn decrement16_test() {
        let mut cpu = Cpu::new();

        cpu.decrement16(RegCode::BC);
        assert_eq!(cpu.registers.bc.take_as_one(), u16::MAX);
    }

    #[test]
    fn jump_relative_test() {
        let mut cpu = Cpu::new();

        cpu.registers.pc = 100;
        cpu.jump_relative(CondCode::Always, -10);
        assert_eq!(cpu.registers.pc, 90);
        cpu.jump_relative(CondCode::Always, 10);
        assert_eq!(cpu.registers.pc, 100);
    }

    #[test]
    fn rotate_left_carry_a_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.left = 0b10000000;
        cpu.rotate_left_carry_a();
        assert!(cpu.registers.af.is_carry_high());

        cpu.registers.af.left = 0b10101010;
        cpu.rotate_left_carry_a();
        assert!(cpu.registers.af.is_carry_high());
        assert_eq!(cpu.registers.af.left, 0b01010101);
    }

    #[test]
    fn rotate_right_carry_a_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.left = 0b00000001;
        cpu.rotate_right_carry_a();
        assert!(cpu.registers.af.is_carry_high());

        cpu.registers.af.left = 0b10101010;
        cpu.rotate_right_carry_a();
        assert_eq!(cpu.registers.af.left, 0b11010101);
    }

    #[test]
    fn rotate_right_a_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.left = 0b00000001;
        cpu.rotate_right_a();
        assert!(cpu.registers.af.is_carry_high());
        assert_eq!(cpu.registers.af.left, 0b10000000);
    }

    #[test]
    fn rotate_left_a_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.left = 0b10000000;
        cpu.rotate_left_a();
        assert!(cpu.registers.af.is_carry_high());
        assert_eq!(cpu.registers.af.left, 0b00000001);
    }

    #[test]
    fn add8_test() {
        let mut cpu = Cpu::new();

        cpu.registers.bc.left = 0b10001000;
        cpu.registers.af.left = 0b10001000;
        cpu.add8(RegCode::B);
        assert!(cpu.registers.af.is_carry_high());
        assert!(cpu.registers.af.is_hcarry_high());
        assert_eq!(cpu.registers.af.left, 0b00010000);

        cpu.registers.af.left = 0;
        cpu.add8(RegCode::Const8(0));
        assert!(cpu.registers.af.is_zero_high());
    }

    #[test]
    fn add16_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.flip_zero_flag();
        cpu.registers.hl.change_as_one(0b1000100000000000);
        cpu.registers.de.change_as_one(0b1000100000000000);
        cpu.add16(RegCode::DE);
        
        assert!(cpu.registers.af.is_zero_high());
        assert!(cpu.registers.af.is_carry_high());
        assert!(cpu.registers.af.is_hcarry_high());
        assert_eq!(cpu.registers.hl.take_as_one(), 0b0001000000000000);
    }

    #[test]
    fn sub_test() {
        let mut cpu = Cpu::new();

        cpu.sub(RegCode::Const8(90));
        
        assert_eq!(cpu.registers.af.left, 166);
        assert!(cpu.registers.af.is_carry_high());

        cpu.sub(RegCode::Const8(166));

        assert_eq!(cpu.registers.af.left, 0);
        assert!(cpu.registers.af.is_zero_high());

        cpu.registers.af.left = 0b00010000;
        cpu.sub(RegCode::Const8(0b00001000));

        assert!(cpu.registers.af.is_hcarry_high());
    }

    #[test]
    fn and_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.left = 0b01010000;
        cpu.and(RegCode::Const8(0b10100000));

        assert!(cpu.registers.af.is_zero_high());
        assert!(cpu.registers.af.is_hcarry_high());
    }

    #[test]
    fn or_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.left = 0b01010000;
        cpu.or(RegCode::Const8(0b01000000));

        assert_eq!(cpu.registers.af.left, 0b01010000);

        cpu.registers.af.left = 0;
        cpu.or(RegCode::Const8(0));

        assert!(cpu.registers.af.is_zero_high());
    }

    #[test]
    fn xor_test() {
        let mut cpu = Cpu::new();

        cpu.registers.af.left = 0b01010000;
        cpu.xor(RegCode::Const8(0b01000000));

        assert_eq!(cpu.registers.af.left, 0b00010000);
        cpu.xor(RegCode::Const8(0b00010000));
        
        assert!(cpu.registers.af.is_zero_high());
    }

    #[test]
    fn addc_test() {
        let mut cpu = Cpu::new();

        cpu.registers.bc.left = 0b10001000;
        cpu.registers.af.left = 0b10001000;
        cpu.addc(RegCode::B);
        assert!(cpu.registers.af.is_carry_high());
        assert!(cpu.registers.af.is_hcarry_high());
        assert_eq!(cpu.registers.af.left, 0b00010000);
        
        cpu.registers.af.flip_flags_down();
        cpu.registers.af.left = 0;
        cpu.addc(RegCode::Const8(0));
        assert!(cpu.registers.af.is_zero_high());

        cpu.registers.af.flip_carry_flag();
        cpu.addc(RegCode::Const8(0));
        assert_eq!(cpu.registers.af.left, 1);
    }

    #[test]
    fn subc_test() {
        let mut cpu = Cpu::new();

        cpu.subc(RegCode::Const8(90));
        
        assert_eq!(cpu.registers.af.left, 166);
        assert!(cpu.registers.af.is_carry_high());
        
        cpu.registers.af.flip_flags_down();
        cpu.subc(RegCode::Const8(166));

        assert_eq!(cpu.registers.af.left, 0);
        assert!(cpu.registers.af.is_zero_high());

        cpu.registers.af.left = 0b00010000;
        cpu.subc(RegCode::Const8(0b00001000));

        assert!(cpu.registers.af.is_hcarry_high());
        
        cpu.registers.af.left = 90;
        cpu.registers.af.flip_carry_flag();
        cpu.subc(RegCode::Const8(89));
        assert!(cpu.registers.af.is_zero_high());
    }

    #[test]
    fn cp_test() {
        let mut cpu = Cpu::new();

        cpu.cp(RegCode::Const8(90));
        
        assert_ne!(cpu.registers.af.left, 166);
        assert!(cpu.registers.af.is_carry_high());
        
        cpu.registers.af.left = 166;
        cpu.cp(RegCode::Const8(166));

        assert_ne!(cpu.registers.af.left, 0);
        assert!(cpu.registers.af.is_zero_high());

        cpu.registers.af.left = 0b00010000;
        cpu.cp(RegCode::Const8(0b00001000));

        assert!(cpu.registers.af.is_hcarry_high());
    }

    #[test]
    fn push_pop_test() {
        let mut cpu = Cpu::new();

        cpu.registers.bc.change_as_one(400);

        cpu.push(RegCode::BC);
        cpu.pop(RegCode::DE);

        assert_eq!(cpu.registers.de.take_as_one(), 400);
    }
}
