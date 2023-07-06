use crate::mos6502::cpu::op_code::OpCode;
use crate::mos6502::cpu::Cpu;
use crate::mos6502::memory::{Memory, RESET_VECTOR};

mod mos6502;

fn main() {
    const PROGRAM: u16 = 0x0600;

    let mut memory = Memory::new();
    memory.write_u16(RESET_VECTOR, PROGRAM);
    // Infinite loop
    memory.write_u8(PROGRAM, OpCode::NopImp as u8);
    memory.write_u8(PROGRAM + 1, OpCode::JmpAbs as u8);
    memory.write_u16(PROGRAM + 2, PROGRAM);

    let mut cpu = Cpu::new(memory);

    for _ in 0..100 {
        cpu.execute();
    }
}
