mod absolute;
mod absolute_x;
mod absolute_y;
mod immediate;

use crate::cpu::CpuRegisters;
use crate::Memory;
use absolute::AbsoluteReadDataState;
use absolute_x::AbsoluteXReadDataState;
use absolute_y::AbsoluteYReadDataState;
use immediate::ImmediateReadDataState;

pub trait AddressingModeRead {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8>;
}

#[derive(Debug, Clone)]
pub enum AddressingModeReadDataState {
    Immediate(ImmediateReadDataState),
    Absolute(AbsoluteReadDataState),
    AbsoluteX(AbsoluteXReadDataState),
    AbsoluteY(AbsoluteYReadDataState),
}

impl AddressingModeRead for AddressingModeReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        use AddressingModeReadDataState::*;

        match self {
            Immediate(state) => state.advance(registers, memory),
            Absolute(state) => state.advance(registers, memory),
            AbsoluteX(state) => state.advance(registers, memory),
            AbsoluteY(state) => state.advance(registers, memory),
        }
    }
}
