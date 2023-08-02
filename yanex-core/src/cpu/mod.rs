mod operation;
mod registers;

pub use operation::Opcode;
pub use operation::Operation;
pub use registers::CpuRegisters;
pub use registers::CpuStatus;
use std::fmt::{Display, Formatter};

use crate::CpuMemory;
use crate::MemoryAccess;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: CpuRegisters,
    pub cycle: u32,
    pub state: CpuState,
}

#[derive(Debug, Default)]
pub struct CpuState(Option<Operation>);

impl Display for CpuState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            None => write!(f, "None"),
            Some(operation) => write!(f, "{}", operation),
        }
    }
}

impl Cpu {
    pub fn next_operation(&mut self, memory: &mut CpuMemory) {
        self.next_cycle(memory);

        while self.state.0.is_some() {
            self.next_cycle(memory)
        }
    }

    pub fn next_cycle(&mut self, memory: &mut CpuMemory) {
        match self.state.0.as_mut() {
            None => self.state.0 = Some(self.fetch_operation(memory)),
            Some(operation) => {
                if (operation).advance(&mut self.registers, memory) {
                    self.state.0 = None
                }
            }
        };

        self.cycle += 1;
    }

    fn fetch_operation(&mut self, memory: &CpuMemory) -> Operation {
        let opcode: Opcode = memory.read_u8(self.registers.program_counter).into();
        self.registers.program_counter += 1;
        opcode.into()
    }
}
