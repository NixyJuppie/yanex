use flags_10::INesHeaderFlags10;
use flags_6::INesHeaderFlags6;
use flags_7::INesHeaderFlags7;
use flags_9::INesHeaderFlags9;

mod flags_10;
mod flags_6;
mod flags_7;
mod flags_9;

pub struct INesHeader {
    pub descriptor: [u8; 4],
    pub b4_prg_rom_size: u8,
    pub b5_chr_rom_size: u8,
    pub b6_flags: INesHeaderFlags6,
    pub b7_flags: INesHeaderFlags7,
    pub b8_prg_ram_size: u8,
    pub b9_flags: INesHeaderFlags9,
    pub b10_flags: INesHeaderFlags10,
    pub b11_unused: u8,
    pub b12_unused: u8,
    pub b13_unused: u8,
    pub b14_unused: u8,
    pub b15_unused: u8,
}

impl TryFrom<[u8; 16]> for INesHeader {
    // todo
    type Error = ();

    fn try_from(value: [u8; 16]) -> Result<Self, Self::Error> {
        // 0..=4 == "NES<MS-DOS EOF>"
        let descriptor: [u8; 4] = [value[0], value[1], value[2], value[3]];

        if value[0..=3] == [0x4E, 0x45, 0x53, 0x1A] {
            Ok(Self {
                descriptor,
                b4_prg_rom_size: value[4],
                b5_chr_rom_size: value[5],
                b6_flags: INesHeaderFlags6::from_bits(value[6]).unwrap(),
                b7_flags: INesHeaderFlags7::from_bits(value[7]).unwrap(),
                b8_prg_ram_size: value[8],
                b9_flags: INesHeaderFlags9::from_bits(value[9]).unwrap(),
                b10_flags: INesHeaderFlags10::from_bits(value[10]).unwrap(),
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
