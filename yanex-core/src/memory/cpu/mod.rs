mod ppu_registers;
mod ram;

use crate::MemoryAccess;
// use cartridge::Cartridge;
use ppu_registers::PpuRegisters;
use ram::Ram;

#[derive(Debug, Default)]
pub struct CpuMemory {
    ram: Ram,
    ppu: PpuRegisters,
    // cartridge: Option<Cartridge>,
}

// impl CpuMemory {
//     pub fn connect_cartridge(&mut self, cartridge: Cartridge) -> Option<Cartridge> {
//         std::mem::replace(&mut self.cartridge, Some(cartridge))
//     }
// }

impl MemoryAccess for CpuMemory {
    fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read_u8(address),
            0x2000..=0x3FFF => self.ppu.read_u8(address),
            0x4000..=0x401F => todo!("APU/IO registers"),
            _ => todo!()
            // 0x4020..=0xFFFF => self
            //     .cartridge
            //     .as_ref()
            //     .unwrap_or_else(|| panic!("Cartridge is not connected"))
            //     .read_u8(address),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write_u8(address, value),
            0x2000..=0x3FFF => self.ppu.write_u8(address, value),
            0x4000..=0x401F => todo!("APU/IO registers"),
            _ => todo!()
            // 
            // 0x4020..=0xFFFF => self
            //     .cartridge
            //     .as_mut()
            //     .unwrap_or_else(|| panic!("Cartridge is not connected"))
            //     .write_u8(address, value),
        };
    }
}
