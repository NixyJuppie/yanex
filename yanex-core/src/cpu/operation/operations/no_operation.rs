use crate::cpu::operation::addressing_mode::AddressingMode;
use crate::cpu::CpuRegisters;
use crate::CpuMemory;

#[derive(Debug, Clone)]
pub enum NoOperationState {
    Decoded(AddressingMode),
    Executed,
}

impl NoOperationState {
    pub fn advance(&mut self, _registers: &mut CpuRegisters, _memory: &mut CpuMemory) -> bool {
        match self {
            NoOperationState::Decoded(ref _mode) => {
                *self = NoOperationState::Executed;
                true
            }
            NoOperationState::Executed => true,
        }
    }
}

// TODO: dummy read and tests
