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
    LoadIndexX(operations::LoadIndexXState),
    LoadIndexY(operations::LoadIndexYState),
}

impl Operation {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) -> bool {
        use Operation::*;

        match self {
            LoadAccumulator(state) => state.advance(registers, memory),
            LoadIndexX(state) => state.advance(registers, memory),
            LoadIndexY(state) => state.advance(registers, memory),
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
            LdxImm => LoadIndexX(LoadIndexXState::Decoded(Immediate)),
            LdxAbs => LoadIndexX(LoadIndexXState::Decoded(Absolute)),
            LdxAbsY => LoadIndexX(LoadIndexXState::Decoded(AbsoluteY)),
            LdxZp => LoadIndexX(LoadIndexXState::Decoded(ZeroPage)),
            LdxZpY => LoadIndexX(LoadIndexXState::Decoded(ZeroPageY)),
            LdyImm => LoadIndexY(LoadIndexYState::Decoded(Immediate)),
            LdyAbs => LoadIndexY(LoadIndexYState::Decoded(Absolute)),
            LdyAbsX => LoadIndexY(LoadIndexYState::Decoded(AbsoluteX)),
            LdyZp => LoadIndexY(LoadIndexYState::Decoded(ZeroPage)),
            LdyZpX => LoadIndexY(LoadIndexYState::Decoded(ZeroPageX)),
            _ => todo!("Unsupported opcode {:?}", opcode),
        }
    }
}
