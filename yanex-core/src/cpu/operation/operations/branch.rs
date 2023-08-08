use super::{mem_read, ReadAddress};
use crate::cpu::{AddressingMode, Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum BranchCarrySet {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchCarrySet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchCarrySet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if !cpu.registers.status.carry() {
                    return Some(());
                }

                match result {
                    None => *self = BranchCarrySet::ReadingAddress(read),
                    Some(address) => *self = BranchCarrySet::Branching(address),
                }
                None
            }
            BranchCarrySet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchCarrySet::Branching(address);
                None
            }
            BranchCarrySet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchCarryClear {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchCarryClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchCarryClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if cpu.registers.status.carry() {
                    return Some(());
                }

                match result {
                    None => *self = BranchCarryClear::ReadingAddress(read),
                    Some(address) => *self = BranchCarryClear::Branching(address),
                }
                None
            }
            BranchCarryClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchCarryClear::Branching(address);
                None
            }
            BranchCarryClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchZeroSet {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchZeroSet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchZeroSet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if !cpu.registers.status.zero() {
                    return Some(());
                }

                match result {
                    None => *self = BranchZeroSet::ReadingAddress(read),
                    Some(address) => *self = BranchZeroSet::Branching(address),
                }
                None
            }
            BranchZeroSet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchZeroSet::Branching(address);
                None
            }
            BranchZeroSet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchZeroClear {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchZeroClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchZeroClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if cpu.registers.status.zero() {
                    return Some(());
                }

                match result {
                    None => *self = BranchZeroClear::ReadingAddress(read),
                    Some(address) => *self = BranchZeroClear::Branching(address),
                }
                None
            }
            BranchZeroClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchZeroClear::Branching(address);
                None
            }
            BranchZeroClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchNegativeSet {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchNegativeSet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchNegativeSet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if !cpu.registers.status.negative() {
                    return Some(());
                }

                match result {
                    None => *self = BranchNegativeSet::ReadingAddress(read),
                    Some(address) => *self = BranchNegativeSet::Branching(address),
                }
                None
            }
            BranchNegativeSet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchNegativeSet::Branching(address);
                None
            }
            BranchNegativeSet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchNegativeClear {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchNegativeClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchNegativeClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if cpu.registers.status.negative() {
                    return Some(());
                }

                match result {
                    None => *self = BranchNegativeClear::ReadingAddress(read),
                    Some(address) => *self = BranchNegativeClear::Branching(address),
                }
                None
            }
            BranchNegativeClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchNegativeClear::Branching(address);
                None
            }
            BranchNegativeClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchOverflowSet {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchOverflowSet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchOverflowSet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if !cpu.registers.status.overflow() {
                    return Some(());
                }

                match result {
                    None => *self = BranchOverflowSet::ReadingAddress(read),
                    Some(address) => *self = BranchOverflowSet::Branching(address),
                }
                None
            }
            BranchOverflowSet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchOverflowSet::Branching(address);
                None
            }
            BranchOverflowSet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchOverflowClear {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    Branching(u16),
}

impl BranchOverflowClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchOverflowClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let result = read.read(cpu, memory);

                if cpu.registers.status.overflow() {
                    return Some(());
                }

                match result {
                    None => *self = BranchOverflowClear::ReadingAddress(read),
                    Some(address) => *self = BranchOverflowClear::Branching(address),
                }
                None
            }
            BranchOverflowClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = BranchOverflowClear::Branching(address);
                None
            }
            BranchOverflowClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}
