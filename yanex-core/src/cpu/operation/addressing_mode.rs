pub use read_address::AddressingModeReadAddress;
pub use read_data::AddressingModeReadData;

mod read_address;
mod read_data;

#[derive(Debug, Clone)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Accumulator,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl AddressingMode {
    pub fn begin_read_address(&self) -> AddressingModeReadAddress {
        match self {
            AddressingMode::Implied => todo!(),
            AddressingMode::Immediate => todo!(),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => AddressingModeReadAddress::ZeroPage(Default::default()),
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
            AddressingMode::Absolute => AddressingModeReadAddress::Absolute(Default::default()),
            AddressingMode::AbsoluteX => todo!(),
            AddressingMode::AbsoluteY => todo!(),
            AddressingMode::Indirect => AddressingModeReadAddress::Indirect(Default::default()),
            AddressingMode::IndirectX => todo!(),
            AddressingMode::IndirectY => todo!(),
        }
    }

    pub fn begin_read_data(&self) -> AddressingModeReadData {
        match self {
            AddressingMode::Implied => AddressingModeReadData::Implied(Default::default()),
            AddressingMode::Immediate => todo!(),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => todo!(),
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
            AddressingMode::Absolute => todo!(),
            AddressingMode::AbsoluteX => todo!(),
            AddressingMode::AbsoluteY => todo!(),
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => todo!(),
            AddressingMode::IndirectY => todo!(),
        }
    }
}
