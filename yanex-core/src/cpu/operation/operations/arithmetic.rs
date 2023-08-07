use super::{mem_read, AddressingModeReadData};
use crate::cpu::{AddressingMode, Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum CompareAccumulator {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl CompareAccumulator {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            CompareAccumulator::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator.wrapping_sub(data);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                cpu.registers
                    .status
                    .set_carry(data <= cpu.registers.accumulator);
                Some(())
            }
            CompareAccumulator::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator.wrapping_sub(data);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                cpu.registers
                    .status
                    .set_carry(data <= cpu.registers.accumulator);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompareIndexX {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl CompareIndexX {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            CompareIndexX::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.index_x.wrapping_sub(data);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                cpu.registers
                    .status
                    .set_carry(data <= cpu.registers.index_x);
                Some(())
            }
            CompareIndexX::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.index_x.wrapping_sub(data);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                cpu.registers
                    .status
                    .set_carry(data <= cpu.registers.index_x);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompareIndexY {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl CompareIndexY {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            CompareIndexY::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.index_y.wrapping_sub(data);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                cpu.registers
                    .status
                    .set_carry(data <= cpu.registers.index_y);
                Some(())
            }
            CompareIndexY::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.index_y.wrapping_sub(data);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);
                cpu.registers
                    .status
                    .set_carry(data <= cpu.registers.index_y);
                Some(())
            }
        }
    }
}
