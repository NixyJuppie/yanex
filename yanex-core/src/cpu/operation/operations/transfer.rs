use super::{mem_read, AddressingMode, AddressingModeReadData};
use crate::cpu::{Cpu, CpuMemory, CpuStatus};

#[derive(Debug, Clone)]
pub enum TransferAccumulatorToX {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl TransferAccumulatorToX {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            TransferAccumulatorToX::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.accumulator,
                    &mut cpu.registers.index_x,
                    &mut cpu.registers.status,
                );
                Some(())
            }
            TransferAccumulatorToX::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.accumulator,
                    &mut cpu.registers.index_x,
                    &mut cpu.registers.status,
                );
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferAccumulatorToY {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl TransferAccumulatorToY {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            TransferAccumulatorToY::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.accumulator,
                    &mut cpu.registers.index_y,
                    &mut cpu.registers.status,
                );
                Some(())
            }
            TransferAccumulatorToY::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.accumulator,
                    &mut cpu.registers.index_y,
                    &mut cpu.registers.status,
                );
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferXToAccumulator {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl TransferXToAccumulator {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            TransferXToAccumulator::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.index_x,
                    &mut cpu.registers.accumulator,
                    &mut cpu.registers.status,
                );
                Some(())
            }
            TransferXToAccumulator::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.index_x,
                    &mut cpu.registers.accumulator,
                    &mut cpu.registers.status,
                );
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferYToAccumulator {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl TransferYToAccumulator {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            TransferYToAccumulator::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.index_y,
                    &mut cpu.registers.accumulator,
                    &mut cpu.registers.status,
                );
                Some(())
            }
            TransferYToAccumulator::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.index_y,
                    &mut cpu.registers.accumulator,
                    &mut cpu.registers.status,
                );
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferStackPointerToX {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl TransferStackPointerToX {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            TransferStackPointerToX::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.stack_pointer,
                    &mut cpu.registers.index_x,
                    &mut cpu.registers.status,
                );
                Some(())
            }
            TransferStackPointerToX::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                transfer(
                    cpu.registers.stack_pointer,
                    &mut cpu.registers.index_x,
                    &mut cpu.registers.status,
                );
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferXToStackPointer {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl TransferXToStackPointer {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            TransferXToStackPointer::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                cpu.registers.stack_pointer = cpu.registers.index_x;
                Some(())
            }
            TransferXToStackPointer::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                cpu.registers.stack_pointer = cpu.registers.index_x;
                Some(())
            }
        }
    }
}

fn transfer(input: u8, output: &mut u8, status: &mut CpuStatus) {
    *output = input;
    status.set_zero(input == 0);
    status.set_negative(input & 0b_1000_0000 > 0);
}
