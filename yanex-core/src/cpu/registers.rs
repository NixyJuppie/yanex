use bitflags::bitflags;

#[derive(Debug, Default, Clone)]
pub struct CpuRegisters {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub status: CpuStatus,
}

bitflags! {
    #[derive(Debug, Clone, Default)]
    pub struct CpuStatus: u8 {
        const B0_CARRY = 0b_0000_0001;
        const B1_ZERO = 0b_0000_0010;
        const B2_INTERRUPT = 0b_0000_0100;
        const B3_DECIMAL = 0b_0000_1000;
        const B4_BREAK = 0b_0001_0000;
        const B5_UNUSED = 0b_0010_0000;
        const B6_OVERFLOW = 0b_0100_0000;
        const B7_NEGATIVE = 0b_1000_0000;
    }
}

impl CpuStatus {
    pub fn set_zero(&mut self, value: u8) {
        self.set(CpuStatus::B1_ZERO, value == 0);
    }

    pub fn set_negative(&mut self, value: u8) {
        self.set(CpuStatus::B7_NEGATIVE, value & 0b1000_0000 != 0);
    }
}
