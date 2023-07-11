use crate::mos6502::cpu::{Cpu, RESET_VECTOR};
use crate::mos6502::memory::Memory;

pub const PROGRAM: u16 = 0x0600;
pub const DATA: u16 = 0xAA00;
pub const DATA_ZP: u16 = 0x00AA;

pub fn data<const I: usize, const O: usize>(input: [u8; I]) -> [u8; O] {
    assert!(O >= I);

    let mut output = [0; O];
    output[..I].copy_from_slice(&input);
    output
}

pub fn init(program: [u8; 100]) -> Cpu {
    init_data(program, [0; 100])
}

pub fn init_data(program: [u8; 100], data: [u8; 100]) -> Cpu {
    init_all(program, data, [0; 100])
}

pub fn init_data_zp(program: [u8; 100], data: [u8; 100]) -> Cpu {
    init_all(program, [0; 100], data)
}

pub fn init_all(program: [u8; 100], data: [u8; 100], data_zp: [u8; 100]) -> Cpu {
    let mut memory = Memory::default();
    for (i, &b) in program.iter().enumerate() {
        memory.write_u8(PROGRAM + i as u16, b);
    }
    for (i, &b) in data.iter().enumerate() {
        memory.write_u8(DATA + i as u16, b);
    }
    for (i, &b) in data_zp.iter().enumerate() {
        memory.write_u8(DATA_ZP + i as u16, b);
    }
    memory.write_u16(RESET_VECTOR, PROGRAM);
    Cpu::new(memory)
}
