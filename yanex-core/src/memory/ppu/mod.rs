use crate::memory::ppu::nametable::NameTable;
use crate::memory::ppu::palette::Palette;
use crate::memory::ppu::patterns::PatternTable;
use crate::MemoryAccess;

mod nametable;
mod palette;
mod patterns;

#[derive(Debug, Default)]
pub struct PpuMemory {
    pattern_table: PatternTable,
    name_table: NameTable,
    palette: Palette,
}

impl MemoryAccess for PpuMemory {
    fn read_u8(&self, address: u16) -> u8 {
        match address & 0x3FFF {
            0x0000..=0x1FFF => self.pattern_table.read_u8(address),
            0x2000..=0x3EFF => self.name_table.read_u8(address),
            0x3F00..=0x3FFF => self.palette.read_u8(address),
            _ => unreachable!(),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address & 0x3FFF {
            0x0000..=0x1FFF => self.pattern_table.write_u8(address, value),
            0x2000..=0x3EFF => self.name_table.write_u8(address, value),
            0x3F00..=0x3FFF => self.palette.write_u8(address, value),
            _ => unreachable!(),
        };
    }
}
