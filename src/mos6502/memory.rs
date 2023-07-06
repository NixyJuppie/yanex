pub const RESET_VECTOR: u16 = 0xFFFC;

#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    data: [u8; 64 * 1024],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 64 * 1024],
        }
    }

    pub fn read_u8(&self, location: u16) -> u8 {
        self.data[location as usize]
    }

    pub fn read_u16(&self, location: u16) -> u16 {
        u16::from_le_bytes([
            self.data[location as usize],
            self.data[(location + 1) as usize],
        ])
    }

    pub fn write_u8(&mut self, location: u16, value: u8) {
        self.data[location as usize] = value;
    }

    pub fn write_u16(&mut self, location: u16, value: u16) {
        self.data[location as usize] = value.to_le_bytes()[0];
        self.data[(location + 1) as usize] = value.to_le_bytes()[1];
    }
}
