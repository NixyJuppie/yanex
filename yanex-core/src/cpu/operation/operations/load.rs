use super::mem_read;
use super::AddressingMode;
use super::AddressingModeReadData;
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum LoadAccumulator {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl LoadAccumulator {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            LoadAccumulator::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                cpu.registers.accumulator = data;
                cpu.registers.status.set_zero_and_negative(data);
                Some(())
            }
            LoadAccumulator::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                cpu.registers.accumulator = data;
                cpu.registers.status.set_zero_and_negative(data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoadIndexX {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl LoadIndexX {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            LoadIndexX::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                cpu.registers.index_x = data;
                cpu.registers.status.set_zero_and_negative(data);
                Some(())
            }
            LoadIndexX::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                cpu.registers.index_x = data;
                cpu.registers.status.set_zero_and_negative(data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoadIndexY {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl LoadIndexY {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            LoadIndexY::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                cpu.registers.index_y = data;
                cpu.registers.status.set_zero_and_negative(data);
                Some(())
            }
            LoadIndexY::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                cpu.registers.index_y = data;
                cpu.registers.status.set_zero_and_negative(data);
                Some(())
            }
        }
    }
}
