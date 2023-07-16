use crate::cpu::Cpu;
use crate::memory::Memory;

mod cpu;
mod memory;

pub struct Nes {
    cpu: Cpu,
    memory: Memory,
}
