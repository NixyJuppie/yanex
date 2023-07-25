use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum AbsoluteYReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u8, u8),
    PageCrossed(u8, u8),
    Data(u8),
}

impl AddressingModeRead for AbsoluteYReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            AbsoluteYReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteYReadDataState::AddressLowByte(low_byte);
                None
            }
            AbsoluteYReadDataState::AddressLowByte(original_low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                let low_byte = original_low_byte.wrapping_add(registers.index_y);

                if low_byte >= *original_low_byte {
                    *self = AbsoluteYReadDataState::Address(low_byte, high_byte);
                } else {
                    // Overflow, page crossed
                    *self = AbsoluteYReadDataState::PageCrossed(low_byte, high_byte);
                }

                None
            }
            AbsoluteYReadDataState::PageCrossed(low_byte, high_byte) => {
                *self = AbsoluteYReadDataState::Address(*low_byte, *high_byte + 1);
                None
            }
            AbsoluteYReadDataState::Address(low_byte, high_byte) => {
                let address = u16::from_le_bytes([*low_byte, *high_byte]);
                let data = memory.read_u8(address);

                *self = AbsoluteYReadDataState::Data(data);
                Some(data)
            }
            AbsoluteYReadDataState::Data(data) => Some(*data),
        }
    }
}
