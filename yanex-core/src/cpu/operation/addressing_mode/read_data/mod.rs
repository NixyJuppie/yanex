mod absolute;
mod absolute_x;
mod absolute_y;
mod immediate;
mod indirect_x;
mod indirect_y;
mod zero_page;
mod zero_page_x;

use crate::cpu::CpuRegisters;
use crate::Memory;
use absolute::AbsoluteReadDataState;
use absolute_x::AbsoluteXReadDataState;
use absolute_y::AbsoluteYReadDataState;
use immediate::ImmediateReadDataState;
use indirect_x::IndirectXReadDataState;
use indirect_y::IndirectYReadDataState;
use zero_page::ZeroPageReadDataState;
use zero_page_x::ZeroPageXReadDataState;

pub trait AddressingModeRead {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8>;
}

#[derive(Debug, Clone)]
pub enum AddressingModeReadDataState {
    Immediate(ImmediateReadDataState),
    Absolute(AbsoluteReadDataState),
    AbsoluteX(AbsoluteXReadDataState),
    AbsoluteY(AbsoluteYReadDataState),
    ZeroPage(ZeroPageReadDataState),
    ZeroPageX(ZeroPageXReadDataState),
    IndirectX(IndirectXReadDataState),
    IndirectY(IndirectYReadDataState),
}

impl AddressingModeRead for AddressingModeReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        use AddressingModeReadDataState::*;

        match self {
            Immediate(state) => state.advance(registers, memory),
            Absolute(state) => state.advance(registers, memory),
            AbsoluteX(state) => state.advance(registers, memory),
            AbsoluteY(state) => state.advance(registers, memory),
            ZeroPage(state) => state.advance(registers, memory),
            ZeroPageX(state) => state.advance(registers, memory),
            IndirectX(state) => state.advance(registers, memory),
            IndirectY(state) => state.advance(registers, memory),
        }
    }
}