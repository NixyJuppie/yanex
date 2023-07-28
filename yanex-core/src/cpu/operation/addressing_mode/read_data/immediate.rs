use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::CpuMemory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum ImmediateReadDataState {
    #[default]
    None,
    Data(u8),
}

impl AddressingModeRead for ImmediateReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            ImmediateReadDataState::None => {
                let data = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ImmediateReadDataState::Data(data);
                Some(data)
            }
            ImmediateReadDataState::Data(data) => Some(*data),
        }
    }
}
