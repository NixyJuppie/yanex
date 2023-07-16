use crate::cpu::registers::CpuRegisters;

mod registers;

#[derive(Debug)]
pub struct Cpu {
    cycle: usize,
    registers: CpuRegisters,
}

#[derive(Debug)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Absolute,
}
