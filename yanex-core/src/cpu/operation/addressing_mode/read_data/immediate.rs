use crate::cpu::operation::addressing_mode::read_data::AddressingModeRead;
use crate::cpu::registers::CpuRegisters;
use crate::memory::memory_access::MemoryAccess;
use crate::memory::Memory;

#[derive(Debug, Default)]
pub enum ImmediateReadDataState {
    #[default]
    None,
    Data(u8),
}

impl AddressingModeRead for ImmediateReadDataState {
    fn advance(self, registers: &mut CpuRegisters, memory: &mut Memory) -> Self {
        match self {
            ImmediateReadDataState::None => {
                let data = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;
                ImmediateReadDataState::Data(data)
            }
            ImmediateReadDataState::Data(_) => unreachable!(),
        }
    }

    fn result(&self) -> Option<u8> {
        match self {
            ImmediateReadDataState::Data(data) => Some(*data),
            _ => None,
        }
    }
}
