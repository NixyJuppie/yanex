use crate::mos6502::cpu::Cpu;
use crate::mos6502::memory::{Memory, RESET_VECTOR};

pub const PROGRAM: u16 = 0x0600;
pub const DATA1: u16 = 0x0700;
pub const DATA2: u16 = 0x0800;

pub fn data<const I: usize, const O: usize>(input: [u8; I]) -> [u8; O] {
    assert!(O >= I);

    let mut output = [0; O];
    output[..I].copy_from_slice(&input);
    output
}

pub fn init(program: [u8; 100]) -> Cpu {
    init_d(program, [0; 100])
}

pub fn init_d(program: [u8; 100], data: [u8; 100]) -> Cpu {
    init_d2(program, data, [0; 100])
}

pub fn init_d2(program: [u8; 100], data1: [u8; 100], data2: [u8; 100]) -> Cpu {
    let mut memory = Memory::new();
    for i in 0..program.len() {
        memory.write(PROGRAM + i as u16, program[i]);
    }
    for i in 0..data1.len() {
        memory.write(DATA1 + i as u16, data1[i]);
    }
    for i in 0..data2.len() {
        memory.write(DATA2 + i as u16, data2[i]);
    }
    memory.write2(RESET_VECTOR, PROGRAM);
    Cpu::new(memory)
}
