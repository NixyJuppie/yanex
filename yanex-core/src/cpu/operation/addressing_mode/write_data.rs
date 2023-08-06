use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeWriteData {
    ZeroPage(u8, ZeroPageAddressingModeWriteData),
}

impl AddressingModeWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            AddressingModeWriteData::ZeroPage(data, state) => state.write(cpu, memory, *data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageAddressingModeWriteData {
    #[default]
    None,
    AddressLowByte(u8),
}

impl ZeroPageAddressingModeWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            ZeroPageAddressingModeWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageAddressingModeWriteData::AddressLowByte(low_byte);
                None
            }
            ZeroPageAddressingModeWriteData::AddressLowByte(low_byte) => {
                let address = u16::from_le_bytes([*low_byte, 0x00]);
                memory.write_u8(address, data);

                Some(())
            }
        }
    }
}
