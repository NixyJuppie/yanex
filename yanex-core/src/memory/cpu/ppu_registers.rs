use super::MemoryAccess;

#[derive(Debug, Default)]
pub struct PpuRegisters {
    data: [u8; 8],
}

impl MemoryAccess for PpuRegisters {
    fn read_u8(&self, address: u16) -> u8 {
        self.data[index(address)]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.data[index(address)] = value
    }
}

fn index(address: u16) -> usize {
    // PPU registers are addressed using first 3 bits, so it can access a maximum of 8B
    (address & 0b_0000_0000_0000_0111) as usize
}
