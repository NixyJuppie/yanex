use crate::Cartridge;

#[derive(Debug)]
pub enum Mapper {
    Nrom,
}

pub enum MappedAddress {
    PrgRom(u8, u16),
}

impl Mapper {
    pub fn map(&self, cartridge: &Cartridge, address: u16) -> MappedAddress {
        match self {
            Mapper::Nrom => NromMapper::map(cartridge, address),
        }
    }
}

#[derive(Debug)]
struct NromMapper;

impl NromMapper {
    pub fn map(cartridge: &Cartridge, address: u16) -> MappedAddress {
        assert!(cartridge.prg_rom.len() <= 2);

        match address {
            0x8000..=0xBFFF => MappedAddress::PrgRom(0, address & 16383),
            0xC000..=0xFFFF => {
                MappedAddress::PrgRom(cartridge.prg_rom.len() as u8 - 1, address & 16383)
            }
            _ => panic!("Address 0x{:04X} is not handled by mapper", address),
        }
    }
}
