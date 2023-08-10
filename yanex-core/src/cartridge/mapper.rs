use super::Cartridge;

#[derive(Debug)]
pub enum Mapper {
    Nrom,
}

impl Mapper {
    pub fn cpu_map(&self, cartridge: &Cartridge, address: u16) -> Option<(u8, u16)> {
        match self {
            Mapper::Nrom => NromMapper::cpu_map(cartridge, address),
        }
    }

    pub fn ppu_map(&self, cartridge: &Cartridge, address: u16) -> Option<(u8, u16)> {
        match self {
            Mapper::Nrom => NromMapper::ppu_map(cartridge, address),
        }
    }
}

#[derive(Debug)]
struct NromMapper;

impl NromMapper {
    pub fn cpu_map(cartridge: &Cartridge, address: u16) -> Option<(u8, u16)> {
        assert!(cartridge.prg_rom.len() <= 2);

        match address {
            0x8000..=0xBFFF => (0, address & 16383).into(),
            0xC000..=0xFFFF => (cartridge.prg_rom.len() as u8 - 1, address & 16383).into(),
            _ => None,
        }
    }

    pub fn ppu_map(cartridge: &Cartridge, _address: u16) -> Option<(u8, u16)> {
        assert!(cartridge.prg_rom.len() <= 2);
        todo!()
    }
}
