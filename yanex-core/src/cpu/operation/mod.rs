use crate::cpu::operation::operations::load::LoadAccumulatorState;
use crate::cpu::registers::CpuRegisters;
use crate::memory::Memory;

pub mod addressing_mode;
pub mod opcode;
mod operations;

#[derive(Debug)]
pub enum Operation {
    LoadAccumulator(LoadAccumulatorState),
}

impl Operation {
    pub fn advance(self, registers: &mut CpuRegisters, memory: &mut Memory) -> Self {
        use Operation::*;

        match self {
            LoadAccumulator(state) => LoadAccumulator(state.advance(registers, memory)),
        }
    }

    pub fn completed(&self) -> bool {
        use Operation::*;

        match self {
            LoadAccumulator(LoadAccumulatorState::Executed) => true,
            _ => false,
        }
    }
}
