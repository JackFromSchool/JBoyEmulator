/*
 *  Utility functions to make my life easier
 */

/// Bit Grabber trait
/// 
/// Takes the nth bit passed in and returns it as a bool
pub trait BitGrabber {
    fn nth_bit_as_bool(&self, n: usize) -> bool;
}

impl BitGrabber for u8 {
    fn nth_bit_as_bool(&self, n: usize) -> bool {
        if to_bits8(*self)[n] {
            true
        } else {
            false
        }
    }
}

impl BitGrabber for u16 {
    fn nth_bit_as_bool(&self, n: usize) -> bool {
        if to_bits16(*self)[n] {
            true
        } else {
            false
        }
    }
}

fn to_bits8(num: u8) -> [bool; 8] {
    let bit_string = format!("{:b}", num);
    let mut bit_ray = [false; 8];
    for (i, bit) in bit_string.chars().rev().enumerate() {
        bit_ray[i] = bit == '1';
    }
    bit_ray
}

fn to_bits16(num: u16) -> [bool; 16] {
    let bit_string = format!("{:b}", num);
    let mut bit_ray = [false; 16];
    for (i, bit) in bit_string.chars().rev().enumerate() {
        bit_ray[i] = bit == '1';
    }
    bit_ray
}
