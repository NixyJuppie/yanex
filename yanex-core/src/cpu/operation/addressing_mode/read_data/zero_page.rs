use super::AddressingModeRead;
use crate::cpu::CpuRegisters;
use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default, Clone)]
pub enum ZeroPageReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Data(u8),
}

impl AddressingModeRead for ZeroPageReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &Memory) -> Option<u8> {
        match self {
            ZeroPageReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ZeroPageReadDataState::AddressLowByte(low_byte);
                None
            }
            ZeroPageReadDataState::AddressLowByte(low_byte) => {
                const HIGH_BYTE: u8 = 0x00;
                let address = u16::from_le_bytes([*low_byte, HIGH_BYTE]);
                let data = memory.read_u8(address);

                *self = ZeroPageReadDataState::Data(data);
                Some(data)
            }
            ZeroPageReadDataState::Data(data) => Some(*data),
        }
    }
}
