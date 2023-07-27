#[derive(Debug, Default, Clone)]
pub struct CpuRegisters {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub status: CpuStatus,
}

#[derive(Debug, Default, Clone)]
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
        self.b1_zero = value == 0
    }

    pub fn set_negative(&mut self, value: u8) {
        self.b7_negative = value & 0b1000_0000 != 0
    }
}

impl From<CpuStatus> for u8 {
    fn from(value: CpuStatus) -> Self {
        let mut result = 0;

        if value.b0_carry {
            result |= 1 << 0
        }

        if value.b1_zero {
            result |= 1 << 1
        }

        if value.b2_interrupt {
            result |= 1 << 2
        }

        if value.b3_decimal {
            result |= 1 << 3
        }

        if value.b4_break {
            result |= 1 << 4
        }

        if value.b5_unused {
            result |= 1 << 5
        }

        if value.b6_overflow {
            result |= 1 << 6
        }

        if value.b7_negative {
            result |= 1 << 7
        }

        result
    }
}

impl From<u8> for CpuStatus {
    fn from(value: u8) -> Self {
        CpuStatus {
            b0_carry: value & 1 << 0 > 0,
            b1_zero: value & 1 << 1 > 0,
            b2_interrupt: value & 1 << 2 > 0,
            b3_decimal: value & 1 << 3 > 0,
            b4_break: value & 1 << 4 > 0,
            b5_unused: value & 1 << 5 > 0,
            b6_overflow: value & 1 << 6 > 0,
            b7_negative: value & 1 << 7 > 0,
        }
    }
}
