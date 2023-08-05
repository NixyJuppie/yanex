mod clear_carry;
mod clear_decimal;
mod clear_interrupt;
mod load_accumulator;
mod load_index_x;
mod load_index_y;
mod no_operation;
mod set_carry;
mod set_decimal;
mod set_interrupt;

use crate::cpu::CpuRegisters;
use crate::CpuMemory;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Operation {
    LoadAccumulator(load_accumulator::LoadAccumulatorState),
    LoadIndexX(load_index_x::LoadIndexXState),
    LoadIndexY(load_index_y::LoadIndexYState),
    NoOperation(no_operation::NoOperationState),
    SetCarry(set_carry::SetCarryState),
    SetDecimal(set_decimal::SetDecimalState),
    SetInterrupt(set_interrupt::SetInterruptState),
    ClearCarry(clear_carry::ClearCarryState),
    ClearDecimal(clear_decimal::ClearDecimalState),
    ClearInterrupt(clear_interrupt::ClearInterruptState),
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::LoadAccumulator(_) => write!(f, "Load Accumulator"),
            Operation::LoadIndexX(_) => write!(f, "Load Index X"),
            Operation::LoadIndexY(_) => write!(f, "Load Index Y"),
            Operation::NoOperation(_) => write!(f, "No Operation"),
            Operation::SetCarry(_) => write!(f, "Set Carry"),
            Operation::SetDecimal(_) => write!(f, "Set Decimal"),
            Operation::SetInterrupt(_) => write!(f, "Set Interrupt"),
            Operation::ClearCarry(_) => write!(f, "Clear Carry"),
            Operation::ClearDecimal(_) => write!(f, "Clear Decimal"),
            Operation::ClearInterrupt(_) => write!(f, "Clear Interrupt"),
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
            SetCarry(state) => state.advance(registers, memory),
            SetDecimal(state) => state.advance(registers, memory),
            SetInterrupt(state) => state.advance(registers, memory),
            ClearCarry(state) => state.advance(registers, memory),
            ClearDecimal(state) => state.advance(registers, memory),
            ClearInterrupt(state) => state.advance(registers, memory),
        }
    }
}

impl From<u8> for Operation {
    fn from(opcode: u8) -> Self {
        use super::AddressingMode::*;
        use Operation::*;

        match opcode {
            0xA9 => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(Immediate)),
            0xAD => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(Absolute)),
            0xA5 => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(ZeroPage)),
            0xB5 => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(ZeroPageX)),
            0xBD => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(AbsoluteX)),
            0xB9 => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(AbsoluteY)),
            0xA1 => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(IndirectX)),
            0xB1 => LoadAccumulator(load_accumulator::LoadAccumulatorState::Decoded(IndirectY)),

            0xA2 => LoadIndexX(load_index_x::LoadIndexXState::Decoded(Immediate)),
            0xA6 => LoadIndexX(load_index_x::LoadIndexXState::Decoded(ZeroPage)),
            0xB6 => LoadIndexX(load_index_x::LoadIndexXState::Decoded(ZeroPageY)),
            0xAE => LoadIndexX(load_index_x::LoadIndexXState::Decoded(Absolute)),
            0xBE => LoadIndexX(load_index_x::LoadIndexXState::Decoded(AbsoluteY)),

            0xA0 => LoadIndexY(load_index_y::LoadIndexYState::Decoded(Immediate)),
            0xA4 => LoadIndexY(load_index_y::LoadIndexYState::Decoded(ZeroPage)),
            0xB4 => LoadIndexY(load_index_y::LoadIndexYState::Decoded(ZeroPageX)),
            0xAC => LoadIndexY(load_index_y::LoadIndexYState::Decoded(Absolute)),
            0xBC => LoadIndexY(load_index_y::LoadIndexYState::Decoded(AbsoluteX)),

            0x38 => SetCarry(set_carry::SetCarryState::Decoded(Implied)),
            0xF8 => SetDecimal(set_decimal::SetDecimalState::Decoded(Implied)),
            0x78 => SetInterrupt(set_interrupt::SetInterruptState::Decoded(Implied)),
            0x18 => ClearCarry(clear_carry::ClearCarryState::Decoded(Implied)),
            0xD8 => ClearDecimal(clear_decimal::ClearDecimalState::Decoded(Implied)),
            0x58 => ClearInterrupt(clear_interrupt::ClearInterruptState::Decoded(Implied)),

            0xEA => NoOperation(no_operation::NoOperationState::Decoded(Implied)),
            _ => todo!("Unsupported opcode 0x{:02X}", opcode),
        }
    }
}
