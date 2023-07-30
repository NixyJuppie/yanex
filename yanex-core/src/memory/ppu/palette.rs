use crate::MemoryAccess;

#[derive(Debug, Default)]
pub struct Palette {
    data: [u8; 32],
}

impl MemoryAccess for Palette {
    fn read_u8(&self, address: u16) -> u8 {
        self.data[index(address)]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.data[index(address)] = value
    }
}

fn index(address: u16) -> usize {
    // Palette is addressed using first 5 bits, so it can access a maximum of 32B
    (address & 0b_0000_0000_0001_1111) as usize
}
