use crate::mos6502::cpu::op_code::OpCode::{RolAbs, RolAbsX, RolAcc, RolZp, RolZpX};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn rol_flags() {
    let mut cpu = init(data([RolAcc as u8, RolAcc as u8]));

    cpu.registers.accumulator = 0b1000_0000;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);

    cpu.registers.accumulator = 0b0111_1111;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
}

#[test]
fn rol_acc() {
    let mut cpu = init(data([RolAcc as u8]));
    cpu.registers.accumulator = 0b1010_1010;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0b0101_0100);
}

#[test]
fn rol_abs() {
    let mut cpu = init_data(
        data([RolAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0b0101_0100);
}

#[test]
fn rol_abs_x() {
    let mut cpu = init_data(
        data([RolAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 4), 0b0101_0100);
}

#[test]
fn rol_zp() {
    let mut cpu = init_data_zp(
        data([RolZp as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0b0101_0100);
}

#[test]
fn rol_zp_x() {
    let mut cpu = init_data_zp(
        data([RolZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 4), 0b0101_0100);
}
