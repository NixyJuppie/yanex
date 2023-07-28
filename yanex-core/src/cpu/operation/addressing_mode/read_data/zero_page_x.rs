use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::CpuMemory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum ZeroPageXReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    DummyRead(u8),
    Data(u8),
}

impl AddressingModeRead for ZeroPageXReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            ZeroPageXReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ZeroPageXReadDataState::AddressLowByte(low_byte);
                None
            }
            ZeroPageXReadDataState::AddressLowByte(low_byte) => {
                memory.read_u8(u16::from_le_bytes([*low_byte, 0x00]));

                *self = ZeroPageXReadDataState::DummyRead(*low_byte);
                None
            }
            ZeroPageXReadDataState::DummyRead(low_byte) => {
                let low_byte = low_byte.wrapping_add(registers.index_x);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                let data = memory.read_u8(address);

                *self = ZeroPageXReadDataState::Data(data);
                Some(data)
            }
            ZeroPageXReadDataState::Data(data) => Some(*data),
        }
    }
}
