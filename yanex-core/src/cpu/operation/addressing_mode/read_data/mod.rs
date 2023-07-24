mod absolute;
mod immediate;

use crate::cpu::operation::addressing_mode::read_data::absolute::AbsoluteReadDataState;
use crate::cpu::operation::addressing_mode::read_data::immediate::ImmediateReadDataState;
use crate::cpu::registers::CpuRegisters;
use crate::memory::Memory;

pub trait AddressingModeRead {
    fn advance(self, registers: &mut CpuRegisters, memory: &mut Memory) -> Self;
    fn result(&self) -> Option<u8>;
}

#[derive(Debug)]
pub enum AddressingModeReadDataState {
    Immediate(ImmediateReadDataState),
    Absolute(AbsoluteReadDataState),
}

impl AddressingModeRead for AddressingModeReadDataState {
    fn advance(self, registers: &mut CpuRegisters, memory: &mut Memory) -> Self {
        use AddressingModeReadDataState::*;

        match self {
            Immediate(state) => Immediate(state.advance(registers, memory)),
            Absolute(state) => Absolute(state.advance(registers, memory)),
        }
    }

    fn result(&self) -> Option<u8> {
        use AddressingModeReadDataState::*;

        match self {
            Immediate(state) => state.result(),
            Absolute(state) => state.result(),
        }
    }
}
