pub use read_address::AddressingModeReadAddress;
pub use read_data::AddressingModeReadData;
pub use write_data::AddressingModeWriteData;

mod read_address;
mod read_data;
mod write_data;

#[derive(Debug, Clone, PartialEq)]
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
            AddressingMode::Relative => AddressingModeReadAddress::Relative(Default::default()),
            AddressingMode::ZeroPage => AddressingModeReadAddress::ZeroPage(Default::default()),
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
            AddressingMode::Absolute => AddressingModeReadAddress::Absolute(Default::default()),
            AddressingMode::AbsoluteX => todo!(),
            AddressingMode::AbsoluteY => todo!(),
            AddressingMode::Indirect => AddressingModeReadAddress::Indirect(Default::default()),
            AddressingMode::IndirectX => AddressingModeReadAddress::IndirectX(Default::default()),
            AddressingMode::IndirectY => AddressingModeReadAddress::IndirectY(Default::default()),
        }
    }

    pub fn begin_read_data(&self) -> AddressingModeReadData {
        match self {
            AddressingMode::Implied => AddressingModeReadData::Implied(Default::default()),
            AddressingMode::Immediate => AddressingModeReadData::Immediate(Default::default()),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => AddressingModeReadData::ZeroPage(Default::default()),
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
            AddressingMode::Absolute => AddressingModeReadData::Absolute(Default::default()),
            AddressingMode::AbsoluteX => AddressingModeReadData::AbsoluteX(Default::default()),
            AddressingMode::AbsoluteY => AddressingModeReadData::AbsoluteY(Default::default()),
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => AddressingModeReadData::IndirectX(Default::default()),
            AddressingMode::IndirectY => AddressingModeReadData::IndirectY(Default::default()),
        }
    }

    pub fn begin_write_data(&self, data: u8) -> AddressingModeWriteData {
        match self {
            AddressingMode::Implied => todo!(),
            AddressingMode::Immediate => todo!(),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => AddressingModeWriteData::ZeroPage(data, Default::default()),
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
            AddressingMode::Absolute => AddressingModeWriteData::Absolute(data, Default::default()),
            AddressingMode::AbsoluteX => {
                AddressingModeWriteData::AbsoluteX(data, Default::default())
            }
            AddressingMode::AbsoluteY => {
                AddressingModeWriteData::AbsoluteY(data, Default::default())
            }
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => {
                AddressingModeWriteData::IndirectX(data, Default::default())
            }
            AddressingMode::IndirectY => {
                AddressingModeWriteData::IndirectY(data, Default::default())
            }
        }
    }
}
