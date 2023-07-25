use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum ZeroPageYReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    DummyRead(u8),
    Data(u8),
}

impl AddressingModeRead for ZeroPageYReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            ZeroPageYReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ZeroPageYReadDataState::AddressLowByte(low_byte);
                None
            }
            ZeroPageYReadDataState::AddressLowByte(low_byte) => {
                memory.read_u8(u16::from_le_bytes([*low_byte, 0x00]));

                *self = ZeroPageYReadDataState::DummyRead(*low_byte);
                None
            }
            ZeroPageYReadDataState::DummyRead(low_byte) => {
                let low_byte = low_byte.wrapping_add(registers.index_y);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                let data = memory.read_u8(address);

                *self = ZeroPageYReadDataState::Data(data);
                Some(data)
            }
            ZeroPageYReadDataState::Data(data) => Some(*data),
        }
    }
}
