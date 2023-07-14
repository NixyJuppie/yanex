use crate::mos6502::cpu::op_code::OpCode::{LsrAbs, LsrAbsX, LsrAcc, LsrZp, LsrZpX};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn flags() {
    let mut cpu = init(data([LsrAcc as u8, LsrAcc as u8]));

    cpu.registers.accumulator = 0b0000_0001;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);

    cpu.registers.accumulator = 0b1111_1110;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
}

#[test]
fn acc() {
    let mut cpu = init(data([LsrAcc as u8]));
    cpu.registers.accumulator = 0b1010_1010;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0b0101_0101);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([LsrAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0b0101_0101);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([LsrAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 4), 0b0101_0101);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([LsrZp as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0b0101_0101);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([LsrZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 4), 0b0101_0101);
}
