use memory_access::MemoryAccess;
use ppu_registers::PpuRegisters;
use ram::Ram;

pub mod memory_access;
pub mod ppu_registers;
pub mod ram;

#[derive(Debug, Default)]
pub struct Memory {
    ram: Ram,
    ppu: PpuRegisters,
}

impl MemoryAccess for Memory {
    fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read_u8(address),
            0x2000..=0x3FFF => self.ppu.read_u8(address),
            _ => todo!(),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write_u8(address, value),
            0x2000..=0x3FFF => self.ram.write_u8(address, value),

            _ => todo!(),
        };
    }
}
