pub struct Memory {
    mem: [u8; 0xFFFF]
}

impl Memory {
    
    pub fn new() -> Self {
        Self {
            mem: [0; 0xFFFF],
        }
    }

}

impl std::ops::Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mem[index]
    }
}

impl std::ops::IndexMut<usize> for Memory {
    
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mem[index]
    }

}
