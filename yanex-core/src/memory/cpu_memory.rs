use super::Cartridge;
use super::MemoryAccess;

#[derive(Debug)]
pub struct CpuMemory {
    ram: [u8; 2048],        // Addressed using 11 bits
    ppu_registers: [u8; 8], // Addressed using 3 bits
    cartridge: Option<Cartridge>,
}

impl CpuMemory {
    pub fn connect_cartridge(&mut self, cartridge: Cartridge) -> Option<Cartridge> {
        std::mem::replace(&mut self.cartridge, Some(cartridge))
    }
}

impl Default for CpuMemory {
    fn default() -> Self {
        Self {
            ram: [0; 2048],
            ppu_registers: [0; 8],
            cartridge: None,
        }
    }
}

impl MemoryAccess for CpuMemory {
    fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize & 2047],
            0x2000..=0x3FFF => self.ppu_registers[address as usize & 7],
            0x4000..=0x401F => todo!("APU/IO registers"),
            0x4020..=0xFFFF => todo!("Catridge"),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram[address as usize & 2047] = value,
            0x2000..=0x3FFF => self.ppu_registers[address as usize & 7] = value,
            0x4000..=0x401F => todo!("APU/IO registers"),
            0x4020..=0xFFFF => todo!("Catridge"),
        };
    }
}
