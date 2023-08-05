use crate::cpu::addressing_mode::{AddressingModeReadData, AddressingModeReadDataState};
use crate::cpu::AddressingMode;
use crate::cpu::CpuRegisters;
use crate::CpuMemory;

#[derive(Debug, Clone)]
pub enum SetDecimalState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl SetDecimalState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
        match self {
            SetDecimalState::Decoded(ref mode) => {
                self.try_execute(registers, memory, (*mode).into())
            }
            SetDecimalState::Fetching(ref mode) => {
                self.try_execute(registers, memory, mode.clone())
            }
            SetDecimalState::Executed => true,
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
                *self = SetDecimalState::Fetching(read_state);
                false
            }
            Some(_) => {
                registers.status.set_decimal(true);

                *self = SetDecimalState::Executed;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_utils::cycles_macros::*;
    use crate::tests_utils::opcode_macros::*;
    use crate::{Cpu, CpuMemory};

    fn assert() -> fn(Cpu, CpuMemory) {
        |mut cpu: Cpu, mut memory: CpuMemory| {
            cpu.next_operation(&mut memory);
            assert!(cpu.registers.status.decimal());
        }
    }

    gen_imp_test!(0xF8, assert());
    gen_imp_cycles_test!(0xF8, 2);
}
