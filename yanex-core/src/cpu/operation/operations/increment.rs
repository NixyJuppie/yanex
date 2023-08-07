use super::{mem_read, AddressingMode, AddressingModeReadData};
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum IncrementIndexX {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl IncrementIndexX {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            IncrementIndexX::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_x.wrapping_add(1);
                cpu.registers.index_x = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
            IncrementIndexX::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_x.wrapping_add(1);
                cpu.registers.index_x = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecrementIndexX {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl DecrementIndexX {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            DecrementIndexX::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_x.wrapping_sub(1);
                cpu.registers.index_x = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
            DecrementIndexX::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_x.wrapping_sub(1);
                cpu.registers.index_x = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum IncrementIndexY {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl IncrementIndexY {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            IncrementIndexY::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_y.wrapping_add(1);
                cpu.registers.index_y = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
            IncrementIndexY::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_y.wrapping_add(1);
                cpu.registers.index_y = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecrementIndexY {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl DecrementIndexY {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            DecrementIndexY::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_y.wrapping_sub(1);
                cpu.registers.index_y = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
            DecrementIndexY::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                let result = cpu.registers.index_y.wrapping_sub(1);
                cpu.registers.index_y = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}
