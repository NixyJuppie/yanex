use crate::mos6502::cpu::op_code::OpCode::{StxAbs, StxZp, StxZpY};
use crate::mos6502::cpu::tests::tests_helpers::{data, init, DATA, DATA_ZP};

#[test]
fn abs() {
    let mut cpu = init(data([
        StxAbs as u8,
        DATA.to_le_bytes()[0],
        DATA.to_le_bytes()[1],
    ]));
    cpu.registers.index_x = 0x69;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x69);
}

#[test]
fn zp() {
    let mut cpu = init(data([StxZp as u8, DATA_ZP.to_le_bytes()[0]]));
    cpu.registers.index_x = 0x69;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0x69);
}

#[test]
fn zp_y() {
    let mut cpu = init(data([StxZpY as u8, DATA_ZP.to_le_bytes()[0]]));
    cpu.registers.index_x = 0x69;
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 4), 0x69);
}
