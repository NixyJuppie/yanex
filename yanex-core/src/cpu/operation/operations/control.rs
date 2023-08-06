use super::mem_read;
use super::AddressingMode;
use super::AddressingModeReadAddress;
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
