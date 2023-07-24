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
    pub fn execute_operation(&mut self, memory: &mut Memory, mut state: Option<Operation>) {
        state = self.execute_cycle(memory, state);

        while state.is_some() {
            state = self.execute_cycle(memory, state)
        }
    }

    pub fn execute_cycle(
        &mut self,
        memory: &mut Memory,
        state: Option<Operation>,
    ) -> Option<Operation> {
        let result = match state {
            None => Some(self.fetch_operation(memory)),
            Some(state) => {
                let result = state.advance(&mut self.registers, memory);
                if result.completed() {
                    None
                } else {
                    Some(result)
                }
            }
        };

        self.cycle += 1;
        result
    }

    fn fetch_operation(&mut self, memory: &Memory) -> Operation {
        let opcode: Opcode = memory.read_u8(self.registers.program_counter).into();
        self.registers.program_counter += 1;
        opcode.into()
    }
}
