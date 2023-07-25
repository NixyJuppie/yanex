use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum IndirectXReadDataState {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
    Address(u8, u8),
    Data(u8),
}

impl AddressingModeRead for IndirectXReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            IndirectXReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = IndirectXReadDataState::PointerLowByte(low_byte);
                None
            }
            IndirectXReadDataState::PointerLowByte(low_byte) => {
                let pointer = u16::from_le_bytes([*low_byte, 0x00]);
                memory.read_u8(pointer);

                *self = IndirectXReadDataState::DummyRead(*low_byte + registers.index_x);
                None
            }
            IndirectXReadDataState::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_low_byte = memory.read_u8(pointer);
                let pointer_low_byte = *pointer_low_byte + 1;

                *self = IndirectXReadDataState::AddressLowByte(pointer_low_byte, address_low_byte);
                None
            }
            IndirectXReadDataState::AddressLowByte(pointer_low_byte, address_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_high_byte = memory.read_u8(pointer);

                *self = IndirectXReadDataState::Address(*address_low_byte, address_high_byte);
                None
            }
            IndirectXReadDataState::Address(low_byte, high_byte) => {
                let address = u16::from_le_bytes([*low_byte, *high_byte]);
                let data = memory.read_u8(address);

                Some(data)
            }
            IndirectXReadDataState::Data(data) => Some(*data),
        }
    }
}
