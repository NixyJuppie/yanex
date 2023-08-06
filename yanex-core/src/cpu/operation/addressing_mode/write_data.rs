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
    AddressLowByte,
}

impl ZeroPageAddressingModeWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            ZeroPageAddressingModeWriteData::None => {
                cpu.internal_registers.address_low_byte =
                    memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageAddressingModeWriteData::AddressLowByte;
                None
            }
            ZeroPageAddressingModeWriteData::AddressLowByte => {
                cpu.internal_registers.address_high_byte = 0x00;
                memory.write_u8(cpu.internal_registers.address(), data);

                Some(())
            }
        }
    }
}
