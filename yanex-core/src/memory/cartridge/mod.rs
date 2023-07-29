use crate::memory::cartridge::ines::INes;

mod ines;

#[derive(Debug, Default)]
pub struct Cartridge {}

impl From<INes> for Cartridge {
    fn from(_value: INes) -> Self {
        todo!()
    }
}
