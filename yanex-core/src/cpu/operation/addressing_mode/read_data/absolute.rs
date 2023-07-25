use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum AbsoluteReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u8, u8),
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

                *self = AbsoluteReadDataState::Address(*low_byte, high_byte);
                None
            }
            AbsoluteReadDataState::Address(low_byte, high_byte) => {
                let address = u16::from_le_bytes([*low_byte, *high_byte]);
                let data = memory.read_u8(address);

                *self = AbsoluteReadDataState::Data(data);
                Some(data)
            }
            AbsoluteReadDataState::Data(data) => Some(*data),
        }
    }
}
