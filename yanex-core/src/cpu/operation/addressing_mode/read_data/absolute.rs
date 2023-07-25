use crate::cpu::operation::addressing_mode::read_data::AddressingModeRead;
use crate::cpu::registers::CpuRegisters;
use crate::memory::memory_access::MemoryAccess;
use crate::memory::Memory;

#[derive(Debug, Default, Clone)]
pub enum AbsoluteReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
    Data(u8),
}

impl AddressingModeRead for AbsoluteReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            AbsoluteReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteReadDataState::AddressLowByte(low_byte);
                None
            }
            AbsoluteReadDataState::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteReadDataState::Address(address);
                None
            }
            AbsoluteReadDataState::Address(address) => {
                let data = memory.read_u8(*address);
                registers.program_counter += 1;

                *self = AbsoluteReadDataState::Data(data);
                Some(data)
            }
            AbsoluteReadDataState::Data(data) => Some(*data),
        }
    }
}
