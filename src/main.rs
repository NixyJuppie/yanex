use crate::cpu::*;
use crate::memory::*;

mod cpu;
mod instruction;
mod memory;

fn main() {
    const START: u16 = 0x0008;

    let mut memory = Memory::new();
    memory.write2(RESET_VECTOR, START);
    // LDA #$69
    memory.write(START, 0xA9);
    memory.write(START + 1, 0x69);
    // STA $01F0
    memory.write(START + 2, 0x8D);
    memory.write2(START + 3, 0x01F0);
    // NOP
    memory.write(START + 5, 0xEA);
    // LDA #$00
    memory.write(START + 6, 0xA9);
    memory.write(START + 7, 0x00);
    // LDA $01F0
    memory.write(START + 8, 0xAD);
    memory.write2(START + 9, 0x01F0);

    let mut cpu = Cpu::new(memory);
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
    assert_eq!(cpu.registers.status.z, false);
    cpu.execute();
    assert_eq!(cpu.memory.read2(0x01F0), 0x69);
    cpu.execute();
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x00);
    assert_eq!(cpu.registers.status.z, true);
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}
