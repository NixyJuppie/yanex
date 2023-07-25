use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum ZeroPageXReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    DeadCycle(u8),
    Data(u8),
}

impl AddressingModeRead for ZeroPageXReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            ZeroPageXReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ZeroPageXReadDataState::AddressLowByte(low_byte);
                None
            }
            ZeroPageXReadDataState::AddressLowByte(low_byte) => {
                // Dummy read
                memory.read_u8(u16::from_le_bytes([*low_byte, 0x00]));

                *self = ZeroPageXReadDataState::DeadCycle(*low_byte);
                None
            }
            ZeroPageXReadDataState::DeadCycle(low_byte) => {
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
