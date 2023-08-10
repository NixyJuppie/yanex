use crate::cartridge::Cartridge;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PpuMemory {
    patterns: [u8; 0x2000],
    nametable: [u8; 0x1000],
    palette: [u8; 0x20],
    cartridge: Rc<Option<Cartridge>>,
}

impl Default for PpuMemory {
    fn default() -> Self {
        Self {
            patterns: [0; 0x2000],
            nametable: [0; 0x1000],
            palette: [0; 0x20],
            cartridge: Default::default(),
        }
    }
}

impl PpuMemory {
    pub fn connect_cartridge(&mut self, cartridge: Rc<Option<Cartridge>>) {
        self.cartridge = cartridge;
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address & 0x3FFF {
            0x0000..=0x1FFF => self.patterns[address as usize],
            0x2000..=0x3EFF => self.nametable[address as usize & 0x1FFF],
            0x3F00..=0x3FFF => self.palette[address as usize & 0x1F],
            _ => unreachable!(),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address & 0x3FFF {
            0x0000..=0x1FFF => self.patterns[address as usize] = data,
            0x2000..=0x3EFF => self.nametable[address as usize & 0x1FFF] = data,
            0x3F00..=0x3FFF => self.palette[address as usize & 0x1F] = data,
            _ => unreachable!(),
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address + 1)])
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, value.to_le_bytes()[0]);
        self.write_u8(address + 1, value.to_le_bytes()[1]);
    }
}
