use crate::MemoryAccess;

#[derive(Debug, Default)]
pub struct Cartridge {
    pub data: Vec<u8>,
}

impl MemoryAccess for Cartridge {
    fn read_u8(&self, address: u16) -> u8 {
        assert!(
            address as usize >= self.data.len(),
            "Cartridge memory is too small to handle address 0x{:04X}",
            address
        );

        // todo
        self.data[address as usize]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        assert!(
            address as usize >= self.data.len(),
            "Cartridge memory is too small to handle address 0x{:04X}",
            address
        );

        // todo
        self.data[address as usize] = value
    }
}
