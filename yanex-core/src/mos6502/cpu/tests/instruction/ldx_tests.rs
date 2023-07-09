use crate::mos6502::cpu::op_code::OpCode::{LdxAbs, LdxAbsY, LdxImm, LdxZp, LdxZpY};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn ldx_flags() {
    let mut cpu = init(data([LdxImm as u8, 0x00, LdxImm as u8, 0xFF]));

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn ldx_imm() {
    let mut cpu = init(data([LdxImm as u8, 0x69]));

    cpu.execute();
    assert_eq!(cpu.registers.index_x, 0x69);
}

#[test]
fn ldx_abs() {
    let mut cpu = init_data(
        data([LdxAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0x69]),
    );

    cpu.execute();
    assert_eq!(cpu.registers.index_x, 0x69);
}

#[test]
fn ldx_abs_y() {
    let mut cpu = init_data(
        data([LdxAbsY as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.index_x, 0x69);
}

#[test]
fn ldx_zp() {
    let mut cpu = init_data_zp(data([LdxZp as u8, DATA_ZP.to_le_bytes()[0]]), data([0x69]));

    cpu.execute();
    assert_eq!(cpu.registers.index_x, 0x69);
}

#[test]
fn ldx_zp_y() {
    let mut cpu = init_data_zp(
        data([LdxZpY as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.index_x, 0x69);
}
