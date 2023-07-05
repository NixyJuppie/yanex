use crate::mos6502::cpu::Cpu;
use crate::mos6502::memory::Memory;

mod mos6502;

fn main() {
    let mut cpu = Cpu::new(Memory::new());
    cpu.execute();
}
