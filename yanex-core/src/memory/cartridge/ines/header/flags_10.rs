use bitflags::bitflags;
bitflags! {
    pub struct INesHeaderFlags10: u8 {
        const B0_TODO = 0b_0000_0001;
        const B1_TODO = 0b_0000_0010;
        const B2_TODO = 0b_0000_0100;
        const B3_TODO = 0b_0000_1000;
        const B4_TODO = 0b_0001_0000;
        const B5_TODO = 0b_0010_0000;
        const B6_TODO = 0b_0100_0000;
        const B7_TODO = 0b_1000_0000;
    }
}
