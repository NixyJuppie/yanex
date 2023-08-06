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
