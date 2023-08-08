use super::{mem_write, AddressingMode, WriteData};
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum StoreAccumulator {
    Decoded(AddressingMode),
    WritingData(WriteData),
}

impl StoreAccumulator {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            StoreAccumulator::Decoded(mode) => {
                let mut write = mode.begin_write_data(cpu.registers.accumulator);
                mem_write!(self, cpu, memory, write, WritingData)
            }
            StoreAccumulator::WritingData(write) => {
                let mut write = write.clone();
                mem_write!(self, cpu, memory, write, WritingData)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum StoreIndexX {
    Decoded(AddressingMode),
    WritingData(WriteData),
}

impl StoreIndexX {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            StoreIndexX::Decoded(mode) => {
                let mut write = mode.begin_write_data(cpu.registers.index_x);
                mem_write!(self, cpu, memory, write, WritingData)
            }
            StoreIndexX::WritingData(write) => {
                let mut write = write.clone();
                mem_write!(self, cpu, memory, write, WritingData)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum StoreIndexY {
    Decoded(AddressingMode),
    WritingData(WriteData),
}

impl StoreIndexY {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            StoreIndexY::Decoded(mode) => {
                let mut write = mode.begin_write_data(cpu.registers.index_y);
                mem_write!(self, cpu, memory, write, WritingData)
            }
            StoreIndexY::WritingData(write) => {
                let mut write = write.clone();
                mem_write!(self, cpu, memory, write, WritingData)
            }
        }
    }
}
