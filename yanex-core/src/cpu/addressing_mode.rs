mod read_address;
mod read_data;

pub use read_address::AddressingModeReadAddress;
pub use read_address::AddressingModeReadAddressState;
pub use read_data::AddressingModeReadData;
pub use read_data::AddressingModeReadDataState;

#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    Implied,
    Immediate,
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

impl From<AddressingMode> for AddressingModeReadAddressState {
    fn from(value: AddressingMode) -> Self {
        use AddressingMode::*;

        match value {
            Implied => todo!(),
            Immediate => todo!(),
            ZeroPage => todo!(),
            ZeroPageX => todo!(),
            ZeroPageY => todo!(),
            Absolute => AddressingModeReadAddressState::Absolute(Default::default()),
            AbsoluteX => todo!(),
            AbsoluteY => todo!(),
            Indirect => AddressingModeReadAddressState::Indirect(Default::default()),
            IndirectX => todo!(),
            IndirectY => todo!(),
        }
    }
}

impl From<AddressingMode> for AddressingModeReadDataState {
    fn from(value: AddressingMode) -> Self {
        use AddressingMode::*;

        match value {
            Implied => AddressingModeReadDataState::Implied(Default::default()),
            Immediate => AddressingModeReadDataState::Immediate(Default::default()),
            ZeroPage => AddressingModeReadDataState::ZeroPage(Default::default()),
            ZeroPageX => AddressingModeReadDataState::ZeroPageX(Default::default()),
            ZeroPageY => AddressingModeReadDataState::ZeroPageY(Default::default()),
            Absolute => AddressingModeReadDataState::Absolute(Default::default()),
            AbsoluteX => AddressingModeReadDataState::AbsoluteX(Default::default()),
            AbsoluteY => AddressingModeReadDataState::AbsoluteY(Default::default()),
            Indirect => todo!(),
            IndirectX => AddressingModeReadDataState::IndirectX(Default::default()),
            IndirectY => AddressingModeReadDataState::IndirectY(Default::default()),
        }
    }
}
