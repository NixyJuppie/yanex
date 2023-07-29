pub struct INesHeader {
    pub b0_n: u8,
    pub b1_e: u8,
    pub b2_s: u8,
    pub b3_eof: u8,
    pub b4_prg_rom_size: u8,
    pub b5_chr_rom_size: u8,
    pub b6_flags: u8,
    pub b7_flags: u8,
    pub b8_flags: u8,
    pub b9_flags: u8,
    pub b10_flags: u8,
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
        if value[0..=4] == [0x4E, 0x45, 0x53, 0x1A] {
            Ok(Self {
                b0_n: value[0],
                b1_e: value[1],
                b2_s: value[2],
                b3_eof: value[3],
                b4_prg_rom_size: value[4],
                b5_chr_rom_size: value[5],
                b6_flags: value[6],
                b7_flags: value[7],
                b8_flags: value[8],
                b9_flags: value[9],
                b10_flags: value[10],
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
