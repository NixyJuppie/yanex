use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum IndirectYReadDataState {
    #[default]
    None,
    PointerLowByte(u8),
    AddressLowByte(u8, u8),
    PageCrossed(u8, u8),
    Address(u8, u8),
    Data(u8),
}

impl AddressingModeRead for IndirectYReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            IndirectYReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = IndirectYReadDataState::PointerLowByte(low_byte);
                None
            }
            IndirectYReadDataState::PointerLowByte(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_low_byte = memory.read_u8(pointer);
                let pointer_low_byte = pointer_low_byte.wrapping_add(1);

                *self = IndirectYReadDataState::AddressLowByte(pointer_low_byte, address_low_byte);
                None
            }
            IndirectYReadDataState::AddressLowByte(pointer_low_byte, original_address_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_high_byte = memory.read_u8(pointer);

                let address_low_byte = original_address_low_byte.wrapping_add(registers.index_y);

                if address_low_byte >= *original_address_low_byte {
                    *self = IndirectYReadDataState::Address(address_low_byte, address_high_byte);
                } else {
                    // Overflow, page crossed
                    *self = IndirectYReadDataState::PageCrossed(address_low_byte, address_high_byte)
                }

                None
            }
            IndirectYReadDataState::PageCrossed(low_byte, high_byte) => {
                *self = IndirectYReadDataState::Address(*low_byte, *high_byte + 1);
                None
            }
            IndirectYReadDataState::Address(low_byte, high_byte) => {
                let address = u16::from_le_bytes([*low_byte, *high_byte]);
                let data = memory.read_u8(address);

                *self = IndirectYReadDataState::Data(data);
                Some(data)
            }
            IndirectYReadDataState::Data(data) => Some(*data),
        }
    }
}
