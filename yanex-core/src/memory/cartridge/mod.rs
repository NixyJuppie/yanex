use crate::memory::cartridge::ines::INes;
use crate::MemoryAccess;

mod ines;

pub trait Mapper {
    fn map(&self, address: u16) -> u16;
}

#[derive(Debug, Default)]
pub struct Cartridge<M: Mapper> {
    mapper: M,
}

impl<M: Mapper> MemoryAccess for Cartridge<M> {
    fn read_u8(&self, _address: u16) -> u8 {
        let _address = self.mapper.map(_address);
        0 // todo
    }

    fn write_u8(&mut self, _address: u16, _value: u8) {
        let _address = self.mapper.map(_address);
    }
}

impl<M: Mapper> From<INes> for Cartridge<M> {
    fn from(_value: INes) -> Self {
        todo!()
    }
}
