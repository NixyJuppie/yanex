use crate::cpu::operation::operations::load::load_accumulator::LoadAccumulatorState;
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
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) -> bool {
        use Operation::*;

        match self {
            LoadAccumulator(state) => state.advance(registers, memory),
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
