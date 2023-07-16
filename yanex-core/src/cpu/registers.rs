#[derive(Debug)]
pub struct CpuRegisters {
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    status: CpuStatus,
}

#[derive(Debug)]
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

impl From<CpuStatus> for u8 {
    fn from(value: CpuStatus) -> Self {
        todo!()
    }
}

impl From<u8> for CpuStatus {
    fn from(value: u8) -> Self {
        todo!()
    }
}
