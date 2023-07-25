use crate::cpu::operation::addressing_mode::read_data::{
    AddressingModeRead, AddressingModeReadDataState,
};
use crate::cpu::operation::addressing_mode::AddressingMode;
use crate::cpu::registers::CpuRegisters;
use crate::memory::Memory;

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
    use crate::cpu::operation::opcode::Opcode::LdaAbs;
    use crate::cpu::Cpu;
    use crate::cpu::Opcode::LdaImm;
    use crate::memory::Memory;
    use crate::tests_utils::*;

    fn assert() -> fn(Cpu, Memory) {
        |mut cpu: Cpu, mut memory: Memory| {
            cpu.execute_operation(&mut memory, &mut None);
            assert_eq!(cpu.registers.accumulator, 0x42);
        }
    }

    gen_imm_tests!(LdaImm, 0x42, 2, assert());
    gen_abs_tests!(LdaAbs, 0x42, 4, assert());
}
