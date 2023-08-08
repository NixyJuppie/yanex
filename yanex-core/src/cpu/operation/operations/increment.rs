use super::{mem_read, AddressingMode, ReadAddress, ReadData};
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum IncrementIndexX {
    Decoded(AddressingMode),
    DummyReadingData(ReadData),
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
    DummyReadingData(ReadData),
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
    DummyReadingData(ReadData),
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
    DummyReadingData(ReadData),
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

#[derive(Debug, Clone)]
pub enum IncrementMemory {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    ReadAddress(u16),
    DummyRead(u16),
    ReadData(u8, u16),
}

impl IncrementMemory {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            IncrementMemory::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = IncrementMemory::ReadAddress(address);
                None
            }
            IncrementMemory::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = IncrementMemory::ReadAddress(address);
                None
            }
            IncrementMemory::ReadAddress(address) => {
                *self = IncrementMemory::DummyRead(*address);
                None
            }
            IncrementMemory::DummyRead(address) => {
                let data = memory.read_u8(*address);

                *self = IncrementMemory::ReadData(data, *address);
                None
            }
            IncrementMemory::ReadData(data, address) => {
                let result = data.wrapping_add(1);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);

                memory.write_u8(*address, result);
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecrementMemory {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    ReadAddress(u16),
    DummyRead(u16),
    ReadData(u8, u16),
}

impl DecrementMemory {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            DecrementMemory::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = DecrementMemory::ReadAddress(address);
                None
            }
            DecrementMemory::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = DecrementMemory::ReadAddress(address);
                None
            }
            DecrementMemory::ReadAddress(address) => {
                *self = DecrementMemory::DummyRead(*address);
                None
            }
            DecrementMemory::DummyRead(address) => {
                let data = memory.read_u8(*address);

                *self = DecrementMemory::ReadData(data, *address);
                None
            }
            DecrementMemory::ReadData(data, address) => {
                let result = data.wrapping_sub(1);
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_negative(result & 0b_1000_0000 > 0);

                memory.write_u8(*address, result);
                Some(())
            }
        }
    }
}
