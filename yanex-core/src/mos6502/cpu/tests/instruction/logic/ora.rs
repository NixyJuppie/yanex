use crate::mos6502::cpu::op_code::OpCode::{
    OraAbs, OraAbsX, OraAbsY, OraImm, OraIndX, OraIndY, OraZp, OraZpX,
};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_all, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn flags() {
    let mut cpu = init(data([
        OraImm as u8,
        0x00,
        OraImm as u8,
        0xFF,
        OraImm as u8,
        0x00,
    ]));

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn imm() {
    let mut cpu = init(data([OraImm as u8, 0x69]));
    cpu.registers.accumulator = 0x42;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([OraAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0x69]),
    );
    cpu.registers.accumulator = 0x42;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([OraAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_x = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}

#[test]
fn abs_y() {
    let mut cpu = init_data(
        data([OraAbsY as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_y = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(data([OraZp as u8, DATA_ZP.to_le_bytes()[0]]), data([0x69]));
    cpu.registers.accumulator = 0x42;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([OraZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_x = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}

#[test]
fn ind_x() {
    let mut cpu = init_all(
        data([OraIndX as u8, (DATA_ZP - 4).to_le_bytes()[0]]),
        data([0x69]),
        data([DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_x = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}

#[test]
fn ind_y() {
    let mut cpu = init_all(
        data([OraIndY as u8, DATA_ZP.to_le_bytes()[0]]),
        data([
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
            0x69,
        ]),
        data([(DATA - 4).to_le_bytes()[0], (DATA - 4).to_le_bytes()[1]]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_y = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x6B);
}
