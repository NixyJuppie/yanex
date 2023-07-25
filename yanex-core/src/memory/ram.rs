use super::MemoryAccess;

#[derive(Debug)]
pub struct Ram {
    data: [u8; 2048],
}

impl Default for Ram {
    fn default() -> Self {
        Self { data: [0; 2048] }
    }
}

impl MemoryAccess for Ram {
    fn read_u8(&self, address: u16) -> u8 {
        self.data[index(address)]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.data[index(address)] = value
    }
}

fn index(address: u16) -> usize {
    // SRAM is addressed using first 11 bits, so it can access a maximum of 2KiB
    (address & 0b0000_0111_1111_1111) as usize
}
