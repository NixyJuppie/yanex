use bitfield_struct::bitfield;

use super::MemoryAccess;

#[derive(Debug)]
pub struct Cartridge {
    mapper: Mapper,
}

impl MemoryAccess for Cartridge {
    fn read_u8(&self, _address: u16) -> u8 {
        let _address = self.mapper.map(_address);
        todo!()
    }

    fn write_u8(&mut self, _address: u16, _value: u8) {
        let _address = self.mapper.map(_address);
        todo!()
    }
}

impl From<INes> for Cartridge {
    fn from(_value: INes) -> Self {
        todo!()
    }
}

#[derive(Debug)]

pub enum Mapper {}

impl Mapper {
    pub fn map(&self, _address: u16) -> u16 {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct INes {
    pub header: INesHeader,
}

#[derive(Debug)]
pub enum TryFromBytes {
    DescriptorNotFound,
    InvalidDescriptor,
}

impl TryFrom<Vec<u8>> for INes {
    type Error = TryFromBytes;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let header_data: [u8; 16] = value[..16]
            .try_into()
            .or(Err(TryFromBytes::DescriptorNotFound))?;

        let header: INesHeader = header_data.try_into()?;

        Ok(Self { header })
    }
}

#[derive(Debug, Default)]
pub struct INesHeader {
    pub descriptor: [u8; 4],
    pub b4_prg_rom_size: u8,
    pub b5_chr_rom_size: u8,
    pub b6_flags: INesHeaderFlags6,
    pub b7_flags: INesHeaderFlags7,
    pub b8_prg_ram_size: u8,
    pub b9_todo: u8,
    pub b10_todo: u8,
    pub b11_unused: u8,
    pub b12_unused: u8,
    pub b13_unused: u8,
    pub b14_unused: u8,
    pub b15_unused: u8,
}

#[bitfield(u8)]
pub struct INesHeaderFlags6 {
    pub horizontal_mirroring: bool,
    pub battery: bool,
    pub trainer: bool,
    pub four_screen: bool,
    #[bits(4)]
    pub mapper_low_nibble: u8,
}

#[bitfield(u8)]
pub struct INesHeaderFlags7 {
    #[bits(4)]
    pub todo: u8,
    #[bits(4)]
    pub mapper_high_nibble: u8,
}

impl TryFrom<[u8; 16]> for INesHeader {
    type Error = TryFromBytes;

    fn try_from(value: [u8; 16]) -> Result<Self, Self::Error> {
        // 0..=4 == "NES<MS-DOS EOF>"
        let descriptor: [u8; 4] = [value[0], value[1], value[2], value[3]];

        if value[0..=3] == [0x4E, 0x45, 0x53, 0x1A] {
            Ok(Self {
                descriptor,
                b4_prg_rom_size: value[4],
                b5_chr_rom_size: value[5],
                b6_flags: value[6].into(),
                b7_flags: value[7].into(),
                b8_prg_ram_size: value[8],
                b9_todo: value[9],
                b10_todo: value[10],
                b11_unused: value[11],
                b12_unused: value[12],
                b13_unused: value[13],
                b14_unused: value[14],
                b15_unused: value[15],
            })
        } else {
            Err(TryFromBytes::InvalidDescriptor)
        }
    }
}
