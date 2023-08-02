mod addressing_mode;
mod opcode;
mod operations;

use crate::cpu::CpuRegisters;
use crate::CpuMemory;
use addressing_mode::AddressingMode;
pub use opcode::Opcode;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Operation {
    LoadAccumulator(operations::LoadAccumulatorState),
    LoadIndexX(operations::LoadIndexXState),
    LoadIndexY(operations::LoadIndexYState),
    NoOperation(operations::NoOperationState),
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::LoadAccumulator(_) => write!(f, "Load Accumulator"),
            Operation::LoadIndexX(_) => write!(f, "Load Index X"),
            Operation::LoadIndexY(_) => write!(f, "Load Index Y"),
            Operation::NoOperation(_) => write!(f, "No Operation"),
        }
    }
}

impl Operation {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
        use Operation::*;

        match self {
            LoadAccumulator(state) => state.advance(registers, memory),
            LoadIndexX(state) => state.advance(registers, memory),
            LoadIndexY(state) => state.advance(registers, memory),
            NoOperation(state) => state.advance(registers, memory),
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
            NopImp => NoOperation(NoOperationState::Decoded(Implied)),
            _ => todo!("Unsupported opcode {:?}", opcode),
        }
    }
}
