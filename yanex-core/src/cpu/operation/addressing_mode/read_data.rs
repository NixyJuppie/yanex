use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeReadData {
    Implied(ImpliedAddressingModeReadData),
    Immediate(ImmediateAddressingModeReadData),
}

impl AddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        match self {
            AddressingModeReadData::Implied(state) => state.read(cpu, memory),
            AddressingModeReadData::Immediate(state) => state.read(cpu, memory),
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
