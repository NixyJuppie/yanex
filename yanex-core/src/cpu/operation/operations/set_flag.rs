use super::{mem_read, AddressingMode, AddressingModeReadData};
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum SetCarry {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl SetCarry {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            SetCarry::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_carry(true);
                Some(())
            }
            SetCarry::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_carry(true);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SetDecimal {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl SetDecimal {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            SetDecimal::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_decimal(true);
                Some(())
            }
            SetDecimal::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_decimal(true);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SetInterrupt {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl SetInterrupt {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            SetInterrupt::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
            SetInterrupt::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
        }
    }
}
