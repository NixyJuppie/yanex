use crate::MemoryAccess;

#[derive(Debug)]
pub struct NameTable {
    data: [u8; 4096],
}

impl Default for NameTable {
    fn default() -> Self {
        Self { data: [0; 4096] }
    }
}

impl MemoryAccess for NameTable {
    fn read_u8(&self, address: u16) -> u8 {
        self.data[index(address)]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.data[index(address)] = value
    }
}

fn index(address: u16) -> usize {
    // Nametable is addressed using first 12 bits, so it can access a maximum of 4KiB
    (address & 0b0000_1111_1111_1111) as usize
}
