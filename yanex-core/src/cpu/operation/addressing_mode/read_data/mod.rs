mod absolute;
mod immediate;

use crate::cpu::CpuRegisters;
use crate::Memory;
use absolute::AbsoluteReadDataState;
use immediate::ImmediateReadDataState;

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
