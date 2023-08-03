use super::MemoryAccess;

#[derive(Debug)]
pub struct PpuMemory {
    patterns: [u8; 8192],  // Addressed using 13 bits
    nametable: [u8; 4096], // Addressed using 12 bits
    palette: [u8; 32],     // Addressed using 5 bits
}

impl Default for PpuMemory {
    fn default() -> Self {
        Self {
            patterns: [0; 8192],
            nametable: [0; 4096],
            palette: [0; 32],
        }
    }
}

impl MemoryAccess for PpuMemory {
    fn read_u8(&self, address: u16) -> u8 {
        match address & 0x3FFF {
            0x0000..=0x1FFF => self.patterns[address as usize & 8191],
            0x2000..=0x3EFF => self.nametable[address as usize & 4095],
            0x3F00..=0x3FFF => self.palette[address as usize & 31],
            _ => unreachable!(),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address & 0x3FFF {
            0x0000..=0x1FFF => self.patterns[address as usize & 8191] = value,
            0x2000..=0x3EFF => self.nametable[address as usize & 4095] = value,
            0x3F00..=0x3FFF => self.palette[address as usize & 31] = value,
            _ => unreachable!(),
        };
    }
}
