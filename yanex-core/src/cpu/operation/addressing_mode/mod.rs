mod read_data;

pub use read_data::AddressingModeRead;
pub use read_data::AddressingModeReadDataState;

#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    Immediate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPage,
    ZeroPageX,
    IndirectX,
    IndirectY,
}

impl From<AddressingMode> for AddressingModeReadDataState {
    fn from(value: AddressingMode) -> Self {
        match value {
            AddressingMode::Immediate => AddressingModeReadDataState::Immediate(Default::default()),
            AddressingMode::Absolute => AddressingModeReadDataState::Absolute(Default::default()),
            AddressingMode::AbsoluteX => AddressingModeReadDataState::AbsoluteX(Default::default()),
            AddressingMode::AbsoluteY => AddressingModeReadDataState::AbsoluteY(Default::default()),
            AddressingMode::ZeroPage => AddressingModeReadDataState::ZeroPage(Default::default()),
            AddressingMode::ZeroPageX => AddressingModeReadDataState::ZeroPageX(Default::default()),
            AddressingMode::IndirectX => AddressingModeReadDataState::IndirectX(Default::default()),
            AddressingMode::IndirectY => AddressingModeReadDataState::IndirectY(Default::default()),
        }
    }
}