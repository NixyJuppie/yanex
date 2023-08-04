use crate::memory::mapper::{MappedAddress, Mapper};
use bitfield_struct::bitfield;

use super::MemoryAccess;

#[derive(Debug)]
pub struct Cartridge {
    pub header: INesHeader,
    pub mapper: Mapper,
    pub trainer: Option<[u8; 512]>,
    pub prg_rom: Vec<[u8; 16384]>,
    pub chr_rom: Vec<[u8; 8192]>,
}

impl MemoryAccess for Cartridge {
    fn read_u8(&self, address: u16) -> u8 {
        match self.mapper.map(self, address) {
            MappedAddress::PrgRom(bank, address) => self.prg_rom[bank as usize][address as usize],
        }
    }

    fn write_u8(&mut self, _address: u16, _value: u8) {
        panic!("Tried writing to cartridge ROM")
    }
}

#[derive(Debug)]
pub enum ParseINesError {
    DescriptorNotFound,
    InvalidDescriptor,
    TrainerNotFound,
    PrgBankNotFound(u8),
    ChrBankNotFound(u8),
}

impl TryFrom<Vec<u8>> for Cartridge {
    type Error = ParseINesError;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let mut i = 0;

        let header: INesHeader = {
            const SIZE: usize = 16;
            let header_data: [u8; SIZE] = data[i..i + SIZE]
                .try_into()
                .or(Err(ParseINesError::DescriptorNotFound))?;
            i += SIZE;

            header_data
                .try_into()
                .or(Err(ParseINesError::InvalidDescriptor))?
        };

        let trainer = if header.b6_flags.trainer() {
            const SIZE: usize = 512;
            let trainer: [u8; SIZE] = data[i..i + SIZE]
                .try_into()
                .or(Err(ParseINesError::TrainerNotFound))?;
            i += SIZE;

            Ok(Some(trainer))
        } else {
            Ok(None)
        }?;

        let prg_rom = {
            const SIZE: usize = 16384;
            let mut prg_rom: Vec<[u8; SIZE]> = Vec::new();

            for bank_index in 0..header.b4_prg_rom_size {
                let prg_bank: [u8; SIZE] = data[i..i + SIZE]
                    .try_into()
                    .or(Err(ParseINesError::ChrBankNotFound(bank_index)))?;
                prg_rom.push(prg_bank);
                i += SIZE;
            }

            prg_rom
        };

        let chr_rom = {
            const SIZE: usize = 8192;
            let mut chr_rom: Vec<[u8; SIZE]> = Vec::new();

            for bank_index in 0..header.b5_chr_rom_size {
                let chr_bank: [u8; SIZE] = data[i..i + SIZE]
                    .try_into()
                    .or(Err(ParseINesError::ChrBankNotFound(bank_index)))?;
                chr_rom.push(chr_bank);
                i += SIZE;
            }

            chr_rom
        };

        // todo

        Ok(Self {
            mapper: header.mapper(),
            header,
            trainer,
            prg_rom,
            chr_rom,
        })
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

impl INesHeader {
    pub fn mapper(&self) -> Mapper {
        let mapper_id =
            self.b6_flags.mapper_low_nibble() | (self.b7_flags.mapper_high_nibble() << 4);

        match mapper_id {
            0x00 => Mapper::Nrom,
            _ => todo!("Not supported mapper id {}", mapper_id),
        }
    }
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
    type Error = ();

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
            Err(())
        }
    }
}
