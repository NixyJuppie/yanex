use crate::cpu::operation::addressing_mode::{
    AddressingMode, AddressingModeRead, AddressingModeReadDataState,
};
use crate::cpu::CpuRegisters;
use crate::Memory;

#[derive(Debug, Clone)]
pub enum LoadAccumulatorState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl LoadAccumulatorState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) -> bool {
        match self {
            LoadAccumulatorState::Decoded(ref mode) => {
                self.try_execute(registers, memory, (*mode).into())
            }
            LoadAccumulatorState::Fetching(ref mode) => {
                self.try_execute(registers, memory, mode.clone())
            }
            LoadAccumulatorState::Executed => true,
        }
    }

    fn try_execute(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &Memory,
        mut read_state: AddressingModeReadDataState,
    ) -> bool {
        match read_state.advance(registers, memory) {
            None => {
                *self = LoadAccumulatorState::Fetching(read_state);
                false
            }
            Some(data) => {
                registers.accumulator = data;
                *self = LoadAccumulatorState::Executed;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_utils::*;
    use crate::Cpu;
    use crate::Memory;
    use crate::Opcode::*;

    fn assert() -> fn(Cpu, Memory) {
        |mut cpu: Cpu, mut memory: Memory| {
            cpu.execute_operation(&mut memory, &mut None);
            assert_eq!(cpu.registers.accumulator, 0x42);
        }
    }

    gen_imm_tests!(LdaImm, 0x42, 2, assert());
    gen_abs_tests!(LdaAbs, 0x42, 4, assert());
}
