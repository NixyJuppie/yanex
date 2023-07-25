use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum AbsoluteXReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u8, u8),
    PageCrossed(u8, u8),
    Data(u8),
}

impl AddressingModeRead for AbsoluteXReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            AbsoluteXReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteXReadDataState::AddressLowByte(low_byte);
                None
            }
            AbsoluteXReadDataState::AddressLowByte(original_low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                let low_byte = original_low_byte.wrapping_add(registers.index_x);

                if low_byte >= *original_low_byte {
                    *self = AbsoluteXReadDataState::Address(low_byte, high_byte);
                } else {
                    // Overflow, page crossed
                    *self = AbsoluteXReadDataState::PageCrossed(low_byte, high_byte);
                }

                None
            }
            AbsoluteXReadDataState::PageCrossed(low_byte, high_byte) => {
                *self = AbsoluteXReadDataState::Address(*low_byte, *high_byte + 1);
                None
            }
            AbsoluteXReadDataState::Address(low_byte, high_byte) => {
                let address = u16::from_le_bytes([*low_byte, *high_byte]);
                let data = memory.read_u8(address);

                *self = AbsoluteXReadDataState::Data(data);
                Some(data)
            }
            AbsoluteXReadDataState::Data(data) => Some(*data),
        }
    }
}
