use super::{mem_read, ReadData};
use crate::cpu::{AddressingMode, Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum CompareAccumulator {
    Decoded(AddressingMode),
    ReadingData(ReadData),
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
    ReadingData(ReadData),
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
    ReadingData(ReadData),
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

#[derive(Debug, Clone)]
pub enum AddCarry {
    Decoded(AddressingMode),
    ReadingData(ReadData),
}

impl AddCarry {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            AddCarry::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                add_with_carry(cpu, data);
                Some(())
            }
            AddCarry::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                add_with_carry(cpu, data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubtractCarry {
    Decoded(AddressingMode),
    ReadingData(ReadData),
}

impl SubtractCarry {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            SubtractCarry::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                add_with_carry(cpu, !data); // Subtract(data) == Add(!data)
                Some(())
            }
            SubtractCarry::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                add_with_carry(cpu, !data); // Subtract(data) == Add(!data)
                Some(())
            }
        }
    }
}

fn add_with_carry(cpu: &mut Cpu, data: u8) {
    // Ignoring decimal flag because NES does not support decimal mode

    let result = data as u16
        + cpu.registers.accumulator as u16
        + if cpu.registers.status.carry() { 1 } else { 0 };

    cpu.registers.status.set_zero(result as u8 == 0);
    cpu.registers.status.set_carry(result > 0b_1111_1111);
    cpu.registers.status.set_negative(!sign(result as u8));
    // Accumulator and data have the same sign but result does not
    cpu.registers.status.set_overflow(
        sign(cpu.registers.accumulator) == sign(data) && sign(data) != sign(result as u8),
    );
    cpu.registers.accumulator = result as u8;
}

fn sign(data: u8) -> bool {
    data & 0b_1000_0000 == 0
}
