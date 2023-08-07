use super::{mem_read, AddressingModeReadData};
use crate::cpu::{AddressingMode, Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum BitTest {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl BitTest {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BitTest::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator & data;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_overflow(data & 0b_0100_0000 > 0);
                cpu.registers.status.set_negative(data & 0b_1000_0000 > 0);
                Some(())
            }
            BitTest::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator & data;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_overflow(data & 0b_0100_0000 > 0);
                cpu.registers.status.set_negative(data & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BitAnd {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl BitAnd {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BitAnd::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator & data;
                cpu.registers.accumulator = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
            BitAnd::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator & data;
                cpu.registers.accumulator = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BitOr {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl BitOr {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BitOr::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator | data;
                cpu.registers.accumulator = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
            BitOr::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator | data;
                cpu.registers.accumulator = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BitXor {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl BitXor {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BitXor::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator ^ data;
                cpu.registers.accumulator = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
            BitXor::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator ^ data;
                cpu.registers.accumulator = result;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}
