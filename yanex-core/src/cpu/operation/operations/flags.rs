use super::{mem_read, AddressingMode, ReadData};
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum SetCarry {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
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
pub enum ClearCarry {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
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
pub enum SetDecimal {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
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
pub enum ClearDecimal {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
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
pub enum SetInterrupt {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
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

#[derive(Debug, Clone)]
pub enum ClearInterrupt {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
}

impl ClearInterrupt {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearInterrupt::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_interrupt(false);
                Some(())
            }
            ClearInterrupt::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_interrupt(false);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClearOverflow {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
}

impl ClearOverflow {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ClearOverflow::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_overflow(false);
                Some(())
            }
            ClearOverflow::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;
                cpu.registers.status.set_overflow(false);
                Some(())
            }
        }
    }
}
