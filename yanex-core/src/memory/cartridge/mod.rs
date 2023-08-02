use crate::MemoryAccess;
pub use ines::INes;
use std::fmt::Debug;

mod ines;

#[derive(Debug)]
pub enum Mapper {}

impl Mapper {
    fn map(&self, _address: u16) -> u16 {
        todo!()
    }
}

#[derive(Debug)]
pub struct Cartridge {
    mapper: Mapper,
}

impl MemoryAccess for Cartridge {
    fn read_u8(&self, _address: u16) -> u8 {
        let _address = self.mapper.map(_address);
        0 // todo
    }

    fn write_u8(&mut self, _address: u16, _value: u8) {
        let _address = self.mapper.map(_address);
    }
}

impl From<INes> for Cartridge {
    fn from(_value: INes) -> Self {
        todo!()
    }
}
