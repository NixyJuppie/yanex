use crate::instruction::*;
use crate::memory::*;

pub struct Cpu {
    pub registers: CpuRegisters,
    pub memory: Memory,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        let mut cpu = Cpu {
            registers: CpuRegisters {
                ..Default::default()
            },
            memory,
        };
        cpu.reset();
        cpu
    }

    pub fn execute(&mut self) {
        let op_code = self.memory.read(self.registers.program_counter);
        self.registers.program_counter += 1;
        execute_instruction(op_code, self);
    }

    pub fn reset(&mut self) {
        self.registers.program_counter = self.memory.read2(RESET_VECTOR);
    }
}

#[derive(Default)]
pub struct CpuRegisters {
    pub program_counter: u16,
    pub accumulator: u8,
    pub status: CpuStatus,
}

#[derive(Clone, Copy, Default)]
pub struct CpuStatus {
    /// Carry flag (bit 0)
    pub c: bool,
    /// Zero flag (bit 1)
    pub z: bool,
    /// Interrupt mask (bit 2)
    pub i: bool,
    /// Decimal flag (bit 3)
    pub d: bool,
    /// Break flag (bit 4)
    pub b: bool,
    /// Unused, always set to 1 (bit 5)
    pub _u: bool,
    /// Overflow flag (bit 6)
    pub v: bool,
    /// Negative flag (bit 7)
    pub n: bool,
}
