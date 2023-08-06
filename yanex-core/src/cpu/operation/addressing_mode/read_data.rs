use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeReadData {
    Implied(ImpliedAddressingModeReadData),
    Immediate(ImmediateAddressingModeReadData),
    ZeroPage(ZeroPageAddressingModeReadData),
}

impl AddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        match self {
            AddressingModeReadData::Implied(state) => state.read(cpu, memory),
            AddressingModeReadData::Immediate(state) => state.read(cpu, memory),
            AddressingModeReadData::ZeroPage(state) => state.read(cpu, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImpliedAddressingModeReadData;

impl ImpliedAddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        // Dummy read
        Some(memory.read_u8(cpu.registers.program_counter))
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImmediateAddressingModeReadData;

impl ImmediateAddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        let data = memory.read_u8(cpu.registers.program_counter);
        cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
        Some(data)
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageAddressingModeReadData {
    #[default]
    None,
    AddressLowByte(u8),
}

impl ZeroPageAddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        match self {
            ZeroPageAddressingModeReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageAddressingModeReadData::AddressLowByte(low_byte);
                None
            }
            ZeroPageAddressingModeReadData::AddressLowByte(low_byte) => {
                let address = u16::from_le_bytes([*low_byte, 0x00]);
                let data = memory.read_u8(address);

                Some(data)
            }
        }
    }
}
