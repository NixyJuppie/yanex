pub use read_address::ReadAddress;
pub use read_data::ReadData;
pub use write_data::WriteData;

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
    pub fn begin_read_address(&self) -> ReadAddress {
        match self {
            AddressingMode::Implied => todo!(),
            AddressingMode::Immediate => todo!(),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Relative => ReadAddress::Relative(Default::default()),
            AddressingMode::ZeroPage => ReadAddress::ZeroPage(Default::default()),
            AddressingMode::ZeroPageX => ReadAddress::ZeroPageX(Default::default()),
            AddressingMode::ZeroPageY => ReadAddress::ZeroPageY(Default::default()),
            AddressingMode::Absolute => ReadAddress::Absolute(Default::default()),
            AddressingMode::AbsoluteX => ReadAddress::AbsoluteX(Default::default()),
            AddressingMode::AbsoluteY => ReadAddress::AbsoluteY(Default::default()),
            AddressingMode::Indirect => ReadAddress::Indirect(Default::default()),
            AddressingMode::IndirectX => ReadAddress::IndirectX(Default::default()),
            AddressingMode::IndirectY => ReadAddress::IndirectY(Default::default()),
        }
    }

    pub fn begin_read_data(&self) -> ReadData {
        match self {
            AddressingMode::Implied => ReadData::Implied(Default::default()),
            AddressingMode::Immediate => ReadData::Immediate(Default::default()),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => ReadData::ZeroPage(Default::default()),
            AddressingMode::ZeroPageX => ReadData::ZeroPageX(Default::default()),
            AddressingMode::ZeroPageY => ReadData::ZeroPageY(Default::default()),
            AddressingMode::Absolute => ReadData::Absolute(Default::default()),
            AddressingMode::AbsoluteX => ReadData::AbsoluteX(Default::default()),
            AddressingMode::AbsoluteY => ReadData::AbsoluteY(Default::default()),
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => ReadData::IndirectX(Default::default()),
            AddressingMode::IndirectY => ReadData::IndirectY(Default::default()),
        }
    }

    pub fn begin_write_data(&self, data: u8) -> WriteData {
        match self {
            AddressingMode::Implied => todo!(),
            AddressingMode::Immediate => todo!(),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => WriteData::ZeroPage(data, Default::default()),
            AddressingMode::ZeroPageX => WriteData::ZeroPageX(data, Default::default()),
            AddressingMode::ZeroPageY => WriteData::ZeroPageY(data, Default::default()),
            AddressingMode::Absolute => WriteData::Absolute(data, Default::default()),
            AddressingMode::AbsoluteX => WriteData::AbsoluteX(data, Default::default()),
            AddressingMode::AbsoluteY => WriteData::AbsoluteY(data, Default::default()),
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => WriteData::IndirectX(data, Default::default()),
            AddressingMode::IndirectY => WriteData::IndirectY(data, Default::default()),
        }
    }
}
