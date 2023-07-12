use crate::mos6502::cpu::op_code::OpCode::{RorAbs, RorAbsX, RorAcc, RorZp, RorZpX};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn ror_flags() {
    let mut cpu = init(data([RorAcc as u8, RorAcc as u8]));

    cpu.registers.accumulator = 0b0000_0001;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);

    cpu.registers.accumulator = 0b1111_1110;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
}

#[test]
fn ror_acc() {
    let mut cpu = init(data([RorAcc as u8]));
    cpu.registers.accumulator = 0b1010_1010;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0b0101_0101);
}

#[test]
fn ror_abs() {
    let mut cpu = init_data(
        data([RorAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0b0101_0101);
}

#[test]
fn ror_abs_x() {
    let mut cpu = init_data(
        data([RorAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 4), 0b0101_0101);
}

#[test]
fn ror_zp() {
    let mut cpu = init_data_zp(
        data([RorZp as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0b1010_1010]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0b0101_0101);
}

#[test]
fn ror_zp_x() {
    let mut cpu = init_data_zp(
        data([RorZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0b1010_1010]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 4), 0b0101_0101);
}
