use crate::cpu::operation::addressing_mode::read_data::AddressingModeReadDataState;

pub mod read_data;

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    Absolute,
}

impl From<AddressingMode> for AddressingModeReadDataState {
    fn from(value: AddressingMode) -> Self {
        match value {
            AddressingMode::Immediate => AddressingModeReadDataState::Immediate(Default::default()),
            AddressingMode::Absolute => AddressingModeReadDataState::Absolute(Default::default()),
        }
    }
}
