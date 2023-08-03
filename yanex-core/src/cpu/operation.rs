mod load;
mod no_operation;

use crate::cpu::CpuRegisters;
use crate::CpuMemory;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Operation {
    LoadAccumulator(load::LoadAccumulatorState),
    LoadIndexX(load::LoadIndexXState),
    LoadIndexY(load::LoadIndexYState),
    NoOperation(no_operation::NoOperationState),
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

impl From<u8> for Operation {
    fn from(opcode: u8) -> Self {
        use super::AddressingMode::*;
        use load::*;
        use no_operation::*;
        use Operation::*;

        match opcode {
            0xA9 => LoadAccumulator(LoadAccumulatorState::Decoded(Immediate)),
            0xAD => LoadAccumulator(LoadAccumulatorState::Decoded(Absolute)),
            0xBD => LoadAccumulator(LoadAccumulatorState::Decoded(AbsoluteX)),
            0xB9 => LoadAccumulator(LoadAccumulatorState::Decoded(AbsoluteY)),
            0xA5 => LoadAccumulator(LoadAccumulatorState::Decoded(ZeroPage)),
            0xB5 => LoadAccumulator(LoadAccumulatorState::Decoded(ZeroPageX)),
            0xA1 => LoadAccumulator(LoadAccumulatorState::Decoded(IndirectX)),
            0xB1 => LoadAccumulator(LoadAccumulatorState::Decoded(IndirectY)),
            0xA2 => LoadIndexX(LoadIndexXState::Decoded(Immediate)),
            0xAE => LoadIndexX(LoadIndexXState::Decoded(Absolute)),
            0xBE => LoadIndexX(LoadIndexXState::Decoded(AbsoluteY)),
            0xA6 => LoadIndexX(LoadIndexXState::Decoded(ZeroPage)),
            0xB6 => LoadIndexX(LoadIndexXState::Decoded(ZeroPageY)),
            0xA0 => LoadIndexY(LoadIndexYState::Decoded(Immediate)),
            0xAC => LoadIndexY(LoadIndexYState::Decoded(Absolute)),
            0xBC => LoadIndexY(LoadIndexYState::Decoded(AbsoluteX)),
            0xA4 => LoadIndexY(LoadIndexYState::Decoded(ZeroPage)),
            0xB4 => LoadIndexY(LoadIndexYState::Decoded(ZeroPageX)),
            0xEA => NoOperation(NoOperationState::Decoded(Implied)),
            _ => todo!("Unsupported opcode {:?}", opcode),
        }
    }
}
