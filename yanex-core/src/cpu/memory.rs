use crate::cartridge::Cartridge;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CpuMemory {
    ram: [u8; 0x800],
    ppu_registers: [u8; 0x8],
    cartridge: Rc<Option<Cartridge>>,
}

impl Default for CpuMemory {
    fn default() -> Self {
        Self {
            ram: [0; 0x800],
            ppu_registers: [0; 0x8],
            cartridge: Default::default(),
        }
    }
}

impl CpuMemory {
    pub fn connect_cartridge(&mut self, cartridge: Rc<Option<Cartridge>>) {
        self.cartridge = cartridge;
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize & 0x7FF],
            0x2000..=0x3FFF => self.ppu_registers[address as usize & 0x7],
            0x4000..=0x401F => todo!("APU/IO registers"),
            0x4020..=0xFFFF => (*self.cartridge).as_ref().unwrap().read_u8(address),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize & 0x7FF] = data,
            0x2000..=0x3FFF => self.ppu_registers[address as usize & 0x7] = data,
            0x4000..=0x401F => todo!("APU/IO registers"),
            0x4020..=0xFFFF => todo!("Cartridge"),
        };
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address + 1)])
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, value.to_le_bytes()[0]);
        self.write_u8(address + 1, value.to_le_bytes()[1]);
    }
}
