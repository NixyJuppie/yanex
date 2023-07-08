use crate::mos6502::cpu::op_code::OpCode::{StyAbs, StyZp, StyZpX};
use crate::mos6502::cpu::tests::tests_helpers::{data, init, DATA, DATA_ZP};

#[test]
fn sty_abs() {
    let mut cpu = init(data([
        StyAbs as u8,
        DATA.to_le_bytes()[0],
        DATA.to_le_bytes()[1],
    ]));
    cpu.registers.index_y = 0x69;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x69);
}

#[test]
fn sty_zp() {
    let mut cpu = init(data([StyZp as u8, DATA_ZP.to_le_bytes()[0]]));
    cpu.registers.index_y = 0x69;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0x69);
}

#[test]
fn sty_zp_x() {
    let mut cpu = init(data([StyZpX as u8, DATA_ZP.to_le_bytes()[0]]));
    cpu.registers.index_y = 0x69;
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 4), 0x69);
}
