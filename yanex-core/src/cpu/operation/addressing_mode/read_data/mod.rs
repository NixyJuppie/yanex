mod absolute;
mod immediate;

use crate::cpu::operation::addressing_mode::read_data::absolute::AbsoluteReadDataState;
use crate::cpu::operation::addressing_mode::read_data::immediate::ImmediateReadDataState;
use crate::cpu::registers::CpuRegisters;
use crate::memory::Memory;

pub trait AddressingModeRead {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8>;
}

#[derive(Debug, Clone)]
pub enum AddressingModeReadDataState {
    Immediate(ImmediateReadDataState),
    Absolute(AbsoluteReadDataState),
}

impl AddressingModeRead for AddressingModeReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        use AddressingModeReadDataState::*;

        match self {
            Immediate(state) => state.advance(registers, memory),
            Absolute(state) => state.advance(registers, memory),
        }
    }
}
