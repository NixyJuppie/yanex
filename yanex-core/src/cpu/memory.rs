use crate::cartridge::Cartridge;

#[derive(Debug, Clone)]
pub struct CpuMemory<'a> {
    ram: [u8; 2048],
    ppu_registers: [u8; 8],
    cartridge: Option<&'a Cartridge>,
}

impl<'a> Default for CpuMemory<'a> {
    fn default() -> Self {
        Self {
            ram: [0; 2048],
            ppu_registers: [0; 8],
            cartridge: None,
        }
    }
}

impl<'a> CpuMemory<'a> {
    pub fn connect_cartridge(&mut self, cartridge: &'a Cartridge) -> Option<&'a Cartridge> {
        std::mem::replace(&mut self.cartridge, Some(cartridge))
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize & 2047],
            0x2000..=0x3FFF => self.ppu_registers[address as usize & 7],
            0x4000..=0x401F => todo!("APU/IO registers"),
            0x4020..=0xFFFF => self.cartridge.unwrap().read_u8(address),
        }
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize & 2047] = data,
            0x2000..=0x3FFF => self.ppu_registers[address as usize & 7] = data,
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
