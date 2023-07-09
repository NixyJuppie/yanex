use crate::mos6502::cpu::op_code::OpCode::{LdyAbs, LdyAbsX, LdyImm, LdyZp, LdyZpX};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn ldy_flags() {
    let mut cpu = init(data([LdyImm as u8, 0x00, LdyImm as u8, 0xFF]));

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn ldy_imm() {
    let mut cpu = init(data([LdyImm as u8, 0x69]));

    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x69);
}

#[test]
fn ldy_abs() {
    let mut cpu = init_data(
        data([LdyAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0x69]),
    );

    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x69);
}

#[test]
fn ldy_abs_x() {
    let mut cpu = init_data(
        data([LdyAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x69);
}

#[test]
fn ldy_zp() {
    let mut cpu = init_data_zp(data([LdyZp as u8, DATA_ZP.to_le_bytes()[0]]), data([0x69]));

    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x69);
}

#[test]
fn ldy_zp_x() {
    let mut cpu = init_data_zp(
        data([LdyZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x69);
}
