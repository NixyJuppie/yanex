#[derive(Debug, Default)]
pub struct CpuRegisters {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub status: CpuStatus,
}

#[derive(Debug, Default)]
pub struct CpuStatus {
    pub b0_carry: bool,
    pub b1_zero: bool,
    pub b2_interrupt: bool,
    pub b3_decimal: bool,
    pub b4_break: bool,
    pub b5_unused: bool,
    pub b6_overflow: bool,
    pub b7_negative: bool,
}

impl CpuStatus {
    pub fn set_zero(&mut self, value: u8) {
        // TODO: negative zero
        self.b1_zero = value == 0
    }

    pub fn set_negative(&mut self, value: u8) {
        self.b7_negative = value & 0b1000_0000 != 0
    }
}

impl From<CpuStatus> for u8 {
    fn from(_value: CpuStatus) -> Self {
        todo!()
    }
}

impl From<u8> for CpuStatus {
    fn from(_value: u8) -> Self {
        todo!()
    }
}
