use crate::cpu::addressing_mode::{AddressingModeReadData, AddressingModeReadDataState};
use crate::cpu::AddressingMode;
use crate::cpu::CpuRegisters;
use crate::CpuMemory;

#[derive(Debug, Clone)]
pub enum NoOperationState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl NoOperationState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
        match self {
            NoOperationState::Decoded(ref mode) => {
                self.try_execute(registers, memory, (*mode).into())
            }
            NoOperationState::Fetching(ref mode) => {
                self.try_execute(registers, memory, mode.clone())
            }
            NoOperationState::Executed => true,
        }
    }

    fn try_execute(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &CpuMemory,
        mut read_state: AddressingModeReadDataState,
    ) -> bool {
        match read_state.advance(registers, memory) {
            None => {
                *self = NoOperationState::Fetching(read_state);
                false
            }
            Some(_) => {
                *self = NoOperationState::Executed;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_utils::cycles_macros::*;
    use crate::tests_utils::opcode_macros::*;

    gen_imp_test!(0xEA, |_, _| {});
    gen_imp_cycles_test!(0xEA, 2);
}
