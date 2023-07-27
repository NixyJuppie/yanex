mod operation;
mod registers;

pub use operation::Opcode;
pub use operation::Operation;
pub use registers::CpuRegisters;
pub use registers::CpuStatus;

use crate::Memory;
use crate::MemoryAccess;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: CpuRegisters,
    pub cycle: u32,
}

impl Cpu {
    pub fn next_operation(&mut self, memory: &mut Memory, state: &mut Option<Operation>) {
        self.next_cycle(memory, state);

        while state.is_some() {
            self.next_cycle(memory, state)
        }
    }

    pub fn next_cycle(&mut self, memory: &mut Memory, state: &mut Option<Operation>) {
        match state {
            None => *state = Some(self.fetch_operation(memory)),
            Some(operation) => {
                if operation.advance(&mut self.registers, memory) {
                    *state = None
                }
            }
        };

        self.cycle += 1;
    }

    fn fetch_operation(&mut self, memory: &Memory) -> Operation {
        let opcode: Opcode = memory.read_u8(self.registers.program_counter).into();
        self.registers.program_counter += 1;
        opcode.into()
    }
}
