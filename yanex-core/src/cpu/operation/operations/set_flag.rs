use super::read;
use super::AddressingMode;
use super::AddressingModeReadData;
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum SetCarry {
    Decoded(AddressingMode),
    DummyRead(AddressingModeReadData),
}

impl SetCarry {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            SetCarry::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_carry(true);
                Some(())
            }
            SetCarry::DummyRead(read) => {
                let mut read = read.clone();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_carry(true);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SetDecimal {
    Decoded(AddressingMode),
    DummyRead(AddressingModeReadData),
}

impl SetDecimal {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            SetDecimal::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_decimal(true);
                Some(())
            }
            SetDecimal::DummyRead(read) => {
                let mut read = read.clone();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_decimal(true);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SetInterrupt {
    Decoded(AddressingMode),
    DummyRead(AddressingModeReadData),
}

impl SetInterrupt {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            SetInterrupt::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
            SetInterrupt::DummyRead(read) => {
                let mut read = read.clone();
                read!(self, cpu, memory, read, DummyRead)?;
                cpu.registers.status.set_interrupt(true);
                Some(())
            }
        }
    }
}
