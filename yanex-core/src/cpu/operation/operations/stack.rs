use super::{mem_read, ReadData};
use crate::cpu::{AddressingMode, Cpu, CpuMemory, CpuStatus};

#[derive(Debug, Clone)]
pub enum PushAccumulator {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
    DummyRead,
}

impl PushAccumulator {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            PushAccumulator::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PushAccumulator::DummyRead;
                None
            }
            PushAccumulator::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PushAccumulator::DummyRead;
                None
            }
            PushAccumulator::DummyRead => {
                cpu.stack_push(memory, cpu.registers.accumulator);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PullAccumulator {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
    DummyRead,
    DeadCycle,
}

impl PullAccumulator {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            PullAccumulator::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PullAccumulator::DummyRead;
                None
            }
            PullAccumulator::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PullAccumulator::DummyRead;
                None
            }
            PullAccumulator::DummyRead => {
                *self = PullAccumulator::DeadCycle;
                None
            }
            PullAccumulator::DeadCycle => {
                let data = cpu.stack_pull(memory);
                cpu.registers.accumulator = data;
                cpu.registers.status.set_zero(data == 0);
                cpu.registers.status.set_negative(data & 0b_1000_0000 > 0);

                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PushStatus {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
    DummyRead,
}

impl PushStatus {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            PushStatus::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PushStatus::DummyRead;
                None
            }
            PushStatus::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PushStatus::DummyRead;
                None
            }
            PushStatus::DummyRead => {
                // Break flag is always pushed as 1
                let data = cpu.registers.status.with_break_(true);
                cpu.stack_push(memory, data.into());
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PullStatus {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
    DummyRead,
    DeadCycle,
}

impl PullStatus {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            PullStatus::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PullStatus::DummyRead;
                None
            }
            PullStatus::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = PullStatus::DummyRead;
                None
            }
            PullStatus::DummyRead => {
                *self = PullStatus::DeadCycle;
                None
            }
            PullStatus::DeadCycle => {
                let mut status: CpuStatus = cpu.stack_pull(memory).into();
                status.set_break_(cpu.registers.status.break_()); // Ignored
                status.set_unused(cpu.registers.status.unused()); // Ignored
                cpu.registers.status = status;
                Some(())
            }
        }
    }
}
