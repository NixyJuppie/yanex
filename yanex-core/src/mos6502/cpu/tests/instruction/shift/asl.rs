use crate::mos6502::cpu::op_code::OpCode::{AslAbs, AslAbsX, AslAcc, AslZp, AslZpX};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn flags() {
    let mut cpu = init(data([AslAcc as u8, AslAcc as u8]));

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
fn acc() {
    let mut cpu = init(data([AslAcc as u8]));
    cpu.registers.accumulator = 0b1010_1010;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0b0101_0100);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([AslAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0b0101_0100);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([AslAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 4), 0b0101_0100);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([AslZp as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0b0101_0100);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([AslZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 4), 0b0101_0100);
}
