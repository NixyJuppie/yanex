use crate::cpu::operation::opcode::Opcode;
use crate::cpu::operation::Operation;
use crate::cpu::registers::CpuRegisters;
use crate::memory::memory_access::MemoryAccess;
use crate::memory::Memory;

pub mod operation;
pub mod registers;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: CpuRegisters,
    pub cycle: u32,
}

#[derive(Debug)]
struct ExecutionContext(Operation);

impl Cpu {
    pub fn execute_operation(&mut self, memory: &mut Memory, state: &mut Option<Operation>) {
        self.execute_cycle(memory, state);

        while state.is_some() {
            self.execute_cycle(memory, state)
        }
    }

    pub fn execute_cycle(&mut self, memory: &mut Memory, state: &mut Option<Operation>) {
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
