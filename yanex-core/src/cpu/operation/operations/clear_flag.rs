use super::read;
use super::AddressingMode;
use super::AddressingModeReadData;
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum ClearCarry {
    Decoded(AddressingMode),
    DummyRead(AddressingModeReadData),
}

impl ClearCarry {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearCarry::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_carry(false);
                Some(())
            }
            ClearCarry::DummyRead(read) => {
                let mut read = read.clone();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_carry(false);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClearDecimal {
    Decoded(AddressingMode),
    DummyRead(AddressingModeReadData),
}

impl ClearDecimal {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearDecimal::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_decimal(false);
                Some(())
            }
            ClearDecimal::DummyRead(read) => {
                let mut read = read.clone();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_decimal(false);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClearInterrupt {
    Decoded(AddressingMode),
    DummyRead(AddressingModeReadData),
}

impl ClearInterrupt {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearInterrupt::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
            ClearInterrupt::DummyRead(read) => {
                let mut read = read.clone();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
        }
    }
}
