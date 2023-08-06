use crate::cpu::operation::addressing_mode::AddressingModeReadAddress;
use crate::cpu::operation::operations::mem_read;
use crate::cpu::{AddressingMode, Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum BranchCarryClear {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchCarryClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchCarryClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.carry() {
                    *self = BranchCarryClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchCarryClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.carry() {
                    *self = BranchCarryClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchCarryClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchCarrySet {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchCarrySet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchCarrySet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.carry() {
                    *self = BranchCarrySet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchCarrySet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.carry() {
                    *self = BranchCarrySet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchCarrySet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchZeroClear {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchZeroClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchZeroClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.zero() {
                    *self = BranchZeroClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchZeroClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.zero() {
                    *self = BranchZeroClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchZeroClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchZeroSet {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchZeroSet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchZeroSet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.zero() {
                    *self = BranchZeroSet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchZeroSet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.zero() {
                    *self = BranchZeroSet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchZeroSet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchNegativeClear {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchNegativeClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchNegativeClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.negative() {
                    *self = BranchNegativeClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchNegativeClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.negative() {
                    *self = BranchNegativeClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchNegativeClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchNegativeSet {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchNegativeSet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchNegativeSet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.negative() {
                    *self = BranchNegativeSet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchNegativeSet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.negative() {
                    *self = BranchNegativeSet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchNegativeSet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchOverflowClear {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchOverflowClear {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchOverflowClear::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.overflow() {
                    *self = BranchOverflowClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchOverflowClear::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if !cpu.registers.status.overflow() {
                    *self = BranchOverflowClear::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchOverflowClear::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BranchOverflowSet {
    Decoded(AddressingMode),
    ReadingAddress(AddressingModeReadAddress),
    Branching(u16),
}

impl BranchOverflowSet {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BranchOverflowSet::Decoded(mode) => {
                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.overflow() {
                    *self = BranchOverflowSet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchOverflowSet::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                if cpu.registers.status.overflow() {
                    *self = BranchOverflowSet::Branching(address);
                    None
                } else {
                    Some(())
                }
            }
            BranchOverflowSet::Branching(address) => {
                cpu.registers.program_counter = *address;
                Some(())
            }
        }
    }
}
