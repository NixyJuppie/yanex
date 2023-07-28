use crate::MemoryAccess;

#[derive(Debug)]
pub struct PatternTable {
    data: [u8; 8192],
}

impl Default for PatternTable {
    fn default() -> Self {
        Self { data: [0; 8192] }
    }
}

impl MemoryAccess for PatternTable {
    fn read_u8(&self, address: u16) -> u8 {
        self.data[index(address)]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.data[index(address)] = value
    }
}

fn index(address: u16) -> usize {
    // PatternTable is addressed using first 13 bits, so it can access a maximum of 8KiB
    (address & 0b0001_1111_1111_1111) as usize
}
