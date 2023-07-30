use bitflags::bitflags;
bitflags! {
    pub struct INesHeaderFlags6: u8 {
        /// 0 = Horizontal, 1 = Vertical
        const B0_MIRRORING = 0b_0000_0001;
        const B1_BATTERY = 0b_0000_0010;
        const B2_TRAINER = 0b_0000_0100;
        const B3_FOUR_SCREEN = 0b_0000_1000;
        const B4_MAPPER_B0 = 0b_0001_0000;
        const B5_MAPPER_B1 = 0b_0010_0000;
        const B6_MAPPER_B2 = 0b_0100_0000;
        const B7_MAPPER_B3 = 0b_1000_0000;
    }
}
