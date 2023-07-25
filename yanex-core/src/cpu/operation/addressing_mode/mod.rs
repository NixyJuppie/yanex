mod read_data;

pub use read_data::AddressingModeRead;
pub use read_data::AddressingModeReadDataState;

#[derive(Debug, Copy, Clone)]
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
