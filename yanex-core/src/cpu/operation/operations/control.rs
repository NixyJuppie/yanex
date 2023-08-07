use super::{mem_read, AddressingMode, AddressingModeReadAddress, AddressingModeReadData};
use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum Jump {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
}

impl Jump {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            Jump::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                cpu.registers.program_counter = mem_read!(self, cpu, memory, read, ReadingAddress)?;
                Some(())
            }
            Jump::ReadingAddress(read) => {
                let mut read = read.clone();
                cpu.registers.program_counter = mem_read!(self, cpu, memory, read, ReadingAddress)?;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum JumpSubroutine {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    DummyRead(u16),
    SubroutineAddress(u16),
    StackHighByte(u16),
}

impl JumpSubroutine {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            JumpSubroutine::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = JumpSubroutine::SubroutineAddress(address);
                None
            }
            JumpSubroutine::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = JumpSubroutine::DummyRead(address);
                None
            }
            JumpSubroutine::DummyRead(address) => {
                // FIXME: Dummy read from stack???

                *self = JumpSubroutine::SubroutineAddress(*address);
                None
            }
            JumpSubroutine::SubroutineAddress(address) => {
                let byte = cpu.registers.program_counter.to_le_bytes()[1];
                cpu.stack_push(memory, byte);

                *self = JumpSubroutine::StackHighByte(*address);
                None
            }
            JumpSubroutine::StackHighByte(address) => {
                let byte = cpu.registers.program_counter.wrapping_sub(1).to_le_bytes()[0];
                cpu.stack_push(memory, byte);
                cpu.registers.program_counter = *address;

                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReturnSubroutine {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
    DeadCycle1,
    DeadCycle2,
    StackLowByte(u8),
    DeadCycle3,
}

impl ReturnSubroutine {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ReturnSubroutine::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = ReturnSubroutine::DeadCycle1;
                None
            }
            ReturnSubroutine::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                *self = ReturnSubroutine::DeadCycle1;
                None
            }
            ReturnSubroutine::DeadCycle1 => {
                // Maybe this should be a dummy read
                *self = ReturnSubroutine::DeadCycle2;
                None
            }
            ReturnSubroutine::DeadCycle2 => {
                let low_byte = cpu.stack_pull(memory).wrapping_add(1);

                *self = ReturnSubroutine::StackLowByte(low_byte);
                None
            }
            ReturnSubroutine::StackLowByte(low_byte) => {
                let high_byte = cpu.stack_pull(memory);
                cpu.registers.program_counter = u16::from_le_bytes([*low_byte, high_byte]);

                *self = ReturnSubroutine::DeadCycle3;
                None
            }
            ReturnSubroutine::DeadCycle3 => {
                // Maybe this should be a dummy read
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum NoOperation {
    Decoded(AddressingMode),
    DummyReadingData(AddressingModeReadData),
}

impl NoOperation {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            NoOperation::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                Some(())
            }
            NoOperation::DummyReadingData(read) => {
                let mut read = read.clone();
                mem_read!(self, cpu, memory, read, DummyReadingData)?;

                Some(())
            }
        }
    }
}
