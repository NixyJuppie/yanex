use crate::cpu::addressing_mode::{AddressingModeRead, AddressingModeReadDataState};
use crate::cpu::AddressingMode;
use crate::cpu::CpuRegisters;
use crate::CpuMemory;

#[derive(Debug, Clone)]
pub enum SetCarryState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl SetCarryState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
        match self {
            SetCarryState::Decoded(ref mode) => self.try_execute(registers, memory, (*mode).into()),
            SetCarryState::Fetching(ref mode) => self.try_execute(registers, memory, mode.clone()),
            SetCarryState::Executed => true,
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
                *self = SetCarryState::Fetching(read_state);
                false
            }
            Some(_) => {
                registers.status.set_carry(true);

                *self = SetCarryState::Executed;
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
            assert!(cpu.registers.status.carry());
        }
    }

    gen_imp_test!(0x38, assert());
    gen_imp_cycles_test!(0x38, 2);
}
