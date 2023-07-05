pub const RESET_VECTOR: u16 = 0xFFFC;

pub struct Memory {
    data: [u8; 64 * 1024],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 64 * 1024],
        }
    }

    pub fn read(&self, location: u16) -> u8 {
        self.data[location as usize]
    }

    pub fn read2(&self, location: u16) -> u16 {
        u16::from_le_bytes([
            self.data[location as usize],
            self.data[(location + 1) as usize],
        ])
    }

    pub fn write(&mut self, location: u16, value: u8) {
        // println!("Memory[0x{:04x}] = 0x{:02x}", location, value);

        self.data[location as usize] = value;
    }

    pub fn write2(&mut self, location: u16, value: u16) {
        // println!("Memory[0x{:04x}] = 0x{:04x}", location, value);

        self.data[location as usize] = value.to_le_bytes()[0];
        self.data[(location + 1) as usize] = value.to_le_bytes()[1];
    }
}
