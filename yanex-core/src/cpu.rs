mod addressing_mode;
mod operation;

use bitfield_struct::bitfield;

pub use addressing_mode::AddressingMode;
pub use operation::Operation;
use std::fmt::{Display, Formatter};

use crate::CpuMemory;
use crate::MemoryAccess;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: CpuRegisters,
    pub cycle: usize,
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
    pub fn reset(&mut self, memory: &CpuMemory) {
        self.registers.accumulator = 0;
        self.registers.index_x = 0;
        self.registers.index_y = 0;
        self.registers.program_counter = memory.read_u16(0xFFFC);
    }

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
        let operation: Operation = memory.read_u8(self.registers.program_counter).into();
        self.registers.program_counter += 1;
        operation
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpuRegisters {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub status: CpuStatus,
}

#[bitfield(u8)]
pub struct CpuStatus {
    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub _break: bool,
    pub unused: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl CpuStatus {
    pub fn set_zero_value(&mut self, value: u8) {
        self.set_zero(value == 0);
    }

    pub fn set_negative_value(&mut self, value: u8) {
        self.set_negative(value & 0b1000_0000 != 0);
    }
}
