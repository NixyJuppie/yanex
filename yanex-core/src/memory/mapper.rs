#[derive(Debug)]
pub enum Mapper {
    Nrom,
}

impl Mapper {
    pub fn map(&self, address: u16) -> u16 {
        match self {
            Mapper::Nrom => match address {
                0x8000..=0xBFFF => address & 16383,
                0xC000..=0xFFFF => address & 16383, // TODO
                _ => todo!(),
            },
        }
    }
}
