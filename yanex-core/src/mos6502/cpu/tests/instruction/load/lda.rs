use crate::mos6502::cpu::op_code::OpCode::{
    LdaAbs, LdaAbsX, LdaAbsY, LdaImm, LdaIndX, LdaIndY, LdaZp, LdaZpX,
};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_all, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn lda_flags() {
    let mut cpu = init(data([LdaImm as u8, 0x00, LdaImm as u8, 0xFF]));

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn lda_imm() {
    let mut cpu = init(data([LdaImm as u8, 0x69]));

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_abs() {
    let mut cpu = init_data(
        data([LdaAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0x69]),
    );

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_abs_x() {
    let mut cpu = init_data(
        data([LdaAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_abs_y() {
    let mut cpu = init_data(
        data([LdaAbsY as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_zp() {
    let mut cpu = init_data_zp(data([LdaZp as u8, DATA_ZP.to_le_bytes()[0]]), data([0x69]));

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_zp_x() {
    let mut cpu = init_data_zp(
        data([LdaZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_ind_x() {
    let mut cpu = init_all(
        data([LdaIndX as u8, (DATA_ZP - 4).to_le_bytes()[0]]),
        data([0x69]),
        data([DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn lda_ind_y() {
    let mut cpu = init_all(
        data([LdaIndY as u8, DATA_ZP.to_le_bytes()[0]]),
        data([
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
            0x69,
        ]),
        data([(DATA - 4).to_le_bytes()[0], (DATA - 4).to_le_bytes()[1]]),
    );
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}
