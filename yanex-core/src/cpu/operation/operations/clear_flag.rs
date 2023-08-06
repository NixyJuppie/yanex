use super::mem_read;
use super::AddressingMode;
use super::AddressingModeReadData;
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum ClearCarry {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl ClearCarry {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearCarry::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_carry(false);
                Some(())
            }
            ClearCarry::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_carry(false);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClearDecimal {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl ClearDecimal {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearDecimal::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_decimal(false);
                Some(())
            }
            ClearDecimal::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_decimal(false);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClearInterrupt {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl ClearInterrupt {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearInterrupt::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
            ClearInterrupt::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
        }
    }
}
