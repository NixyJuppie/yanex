use crate::cpu::operation::addressing_mode::read_data::AddressingModeRead;
use crate::cpu::registers::CpuRegisters;
use crate::memory::memory_access::MemoryAccess;
use crate::memory::Memory;

#[derive(Debug, Default)]
pub enum AbsoluteReadDataState {
    #[default]
    None,
    LowByteAddress(u8),
    Address(u16),
    Data(u8),
}

impl AddressingModeRead for AbsoluteReadDataState {
    fn advance(self, registers: &mut CpuRegisters, memory: &mut Memory) -> Self {
        match self {
            AbsoluteReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;
                AbsoluteReadDataState::LowByteAddress(low_byte)
            }
            AbsoluteReadDataState::LowByteAddress(low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;
                AbsoluteReadDataState::Address(u16::from_le_bytes([low_byte, high_byte]))
            }
            AbsoluteReadDataState::Address(address) => {
                let data = memory.read_u8(address);
                registers.program_counter += 1;
                AbsoluteReadDataState::Data(data)
            }
            AbsoluteReadDataState::Data(_) => unreachable!(),
        }
    }

    fn result(&self) -> Option<u8> {
        match self {
            AbsoluteReadDataState::Data(data) => Some(*data),
            _ => None,
        }
    }
}
