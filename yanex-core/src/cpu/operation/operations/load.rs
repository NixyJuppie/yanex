use crate::cpu::operation::addressing_mode::read_data::{
    AddressingModeRead, AddressingModeReadDataState,
};
use crate::cpu::operation::addressing_mode::AddressingMode;
use crate::cpu::registers::CpuRegisters;
use crate::memory::Memory;

#[derive(Debug)]
pub enum LoadAccumulatorState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Fetched(u8),
    Executed,
}

impl LoadAccumulatorState {
    pub fn advance(self, registers: &mut CpuRegisters, memory: &mut Memory) -> Self {
        match self {
            LoadAccumulatorState::Decoded(mode) => {
                let mut read_state: AddressingModeReadDataState = mode.into();
                read_state = read_state.advance(registers, memory);
                LoadAccumulatorState::Fetching(read_state)
            }
            LoadAccumulatorState::Fetching(state) => {
                let result = state.result();

                match result {
                    None => LoadAccumulatorState::Fetching(state.advance(registers, memory)),
                    Some(data) => LoadAccumulatorState::Fetched(data),
                }
            }
            LoadAccumulatorState::Fetched(data) => {
                registers.accumulator = data;
                LoadAccumulatorState::Executed
            }
            LoadAccumulatorState::Executed => unreachable!(),
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
            cpu.execute_operation(&mut memory, None);
            assert_eq!(cpu.registers.accumulator, 0x42);
        }
    }

    gen_imm_tests!(LdaImm, 0x42, 2, assert());
    gen_abs_tests!(LdaAbs, 0x42, 4, assert());
}
