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
        if self.to_le_bytes()[n] == 1 {
            true
        } else {
            false
        }
    }
}

impl BitGrabber for u16 {
    fn nth_bit_as_bool(&self, n: usize) -> bool {
        if self.to_le_bytes()[n] == 1 {
            true
        } else {
            false
        }
    }
}
