use num_traits::FromPrimitive;

use crate::mos6502::cpu::instruction::execute_instruction;
use crate::mos6502::cpu::op_code::OpCode;
use crate::mos6502::memory::Memory;

mod addressing_mode;
mod instruction;
pub mod op_code;

#[cfg(test)]
mod tests;

pub const RESET_VECTOR: u16 = 0xFFFC;
pub const STACK_LOCATION: u16 = 0x01000;

#[derive(Debug, Clone, PartialEq)]
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
        let op_code = self.memory.read_u8(self.registers.program_counter);
        self.registers.program_counter += 1;
        let op_code = OpCode::from_u8(op_code).expect("OpCode not found");
        execute_instruction(op_code, self);
    }

    pub fn reset(&mut self) {
        self.registers.program_counter = self.memory.read_u16(RESET_VECTOR);
        self.registers.index_x = 0x00;
        self.registers.index_y = 0x00;
        self.registers.stack_pointer = 0xFF;
    }

    pub fn read_stack(&self, offset: u8) -> u8 {
        self.memory.read_u8(STACK_LOCATION + offset as u16)
    }

    pub fn write_stack(&mut self, offset: u8, value: u8) {
        self.memory.write_u8(STACK_LOCATION + offset as u16, value);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CpuRegisters {
    pub program_counter: u16,
    pub accumulator: u8,
    pub stack_pointer: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub status: CpuStatus,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
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
    /// Unused (bit 5)
    pub u: bool,
    /// Overflow flag (bit 6)
    pub v: bool,
    /// Negative flag (bit 7)
    pub n: bool,
}

impl From<u8> for CpuStatus {
    fn from(value: u8) -> Self {
        Self {
            c: value & 0b00000001 > 0,
            z: value & 0b00000010 > 0,
            i: value & 0b00000010 > 0,
            d: value & 0b00000100 > 0,
            b: value & 0b00001000 > 0,
            u: value & 0b00010000 > 0,
            v: value & 0b01000000 > 0,
            n: value & 0b10000000 > 0,
        }
    }
}

impl From<CpuStatus> for u8 {
    fn from(status: CpuStatus) -> Self {
        let mut value = 0;

        if status.c {
            value |= 0b0000_0001
        }
        if status.z {
            value |= 0b0000_0010
        }
        if status.i {
            value |= 0b0000_0100
        }
        if status.d {
            value |= 0b0000_1000
        }
        if status.b {
            value |= 0b0001_0000
        }
        if status.u {
            value |= 0b0010_0000
        }
        if status.v {
            value |= 0b0100_0000
        }
        if status.n {
            value |= 0b1000_0000
        }

        value
    }
}
