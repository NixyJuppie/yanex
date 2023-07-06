use crate::mos6502::cpu::op_code::OpCode::{LdaAbs, LdaAbsX, LdaAbsY, LdaImm};
use crate::mos6502::cpu::tests::tests_helpers::{data, init, init_d, DATA1};

#[test]
fn lda_flags() {
    let mut cpu = init(data([LdaImm as u8, 0xff, LdaImm as u8, 0x00]));

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
}

#[test]
fn lda_imm() {
    let mut cpu = init(data([LdaImm as u8, 0x10]));

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x10);
}

#[test]
fn lda_abs() {
    let mut cpu = init_d(
        data([LdaAbs as u8, DATA1.to_le_bytes()[0], DATA1.to_le_bytes()[1]]),
        data([0x12]),
    );

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x12);
}

#[test]
fn lda_abs_x() {
    let mut cpu = init_d(
        data([
            LdaAbsX as u8,
            DATA1.to_le_bytes()[0],
            DATA1.to_le_bytes()[1],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_abs_y() {
    let mut cpu = init_d(
        data([
            LdaAbsY as u8,
            DATA1.to_le_bytes()[0],
            DATA1.to_le_bytes()[1],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}
