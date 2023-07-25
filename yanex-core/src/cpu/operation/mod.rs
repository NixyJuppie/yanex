mod addressing_mode;
mod opcode;
mod operations;

use crate::cpu::CpuRegisters;
use crate::Memory;
use addressing_mode::AddressingMode;
pub use opcode::Opcode;

#[derive(Debug)]
pub enum Operation {
    LoadAccumulator(operations::LoadAccumulatorState),
}

impl Operation {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) -> bool {
        use Operation::*;

        match self {
            LoadAccumulator(state) => state.advance(registers, memory),
        }
    }
}

impl From<Opcode> for Operation {
    fn from(opcode: Opcode) -> Self {
        use operations::*;
        use AddressingMode::*;
        use Opcode::*;
        use Operation::*;

        match opcode {
            LdaImm => LoadAccumulator(LoadAccumulatorState::Decoded(Immediate)),
            LdaAbs => LoadAccumulator(LoadAccumulatorState::Decoded(Absolute)),
            LdaAbsX => LoadAccumulator(LoadAccumulatorState::Decoded(AbsoluteX)),
            LdaAbsY => LoadAccumulator(LoadAccumulatorState::Decoded(AbsoluteY)),
            LdaZp => LoadAccumulator(LoadAccumulatorState::Decoded(ZeroPage)),
            LdaZpX => LoadAccumulator(LoadAccumulatorState::Decoded(ZeroPageX)),
            LdaIndX => LoadAccumulator(LoadAccumulatorState::Decoded(IndirectX)),
            LdaIndY => LoadAccumulator(LoadAccumulatorState::Decoded(IndirectY)),
            _ => todo!("Unsupported opcode {:?}", opcode),
        }
    }
}
