use crate::mos6502::cpu::op_code::OpCode::{
    AndAbs, AndAbsX, AndAbsY, AndImm, AndIndX, AndIndY, AndZp, AndZpX,
};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_all, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn flags() {
    let mut cpu = init(data([AndImm as u8, 0x00, AndImm as u8, 0xFF]));

    cpu.registers.accumulator = 0b1111_1111;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.registers.accumulator = 0b1111_1111;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn imm() {
    let mut cpu = init(data([AndImm as u8, 0x69]));
    cpu.registers.accumulator = 0x42;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x40);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([AndAbs as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0x69]),
    );
    cpu.registers.accumulator = 0x42;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x40);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([AndAbsX as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_x = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x40);
}

#[test]
fn abs_y() {
    let mut cpu = init_data(
        data([AndAbsY as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_y = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x40);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(data([AndZp as u8, DATA_ZP.to_le_bytes()[0]]), data([0x69]));
    cpu.registers.accumulator = 0x42;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x40);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([AndZpX as u8, DATA_ZP.to_le_bytes()[0]]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_x = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x40);
}

#[test]
fn ind_x() {
    let mut cpu = init_all(
        data([AndIndX as u8, (DATA_ZP - 4).to_le_bytes()[0]]),
        data([0x69]),
        data([DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
    );
    cpu.registers.accumulator = 0x42;
    cpu.registers.index_x = 4;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x40);
}

#[test]
fn ind_y() {
    let mut cpu = init_all(
        data([AndIndY as u8, DATA_ZP.to_le_bytes()[0]]),
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
    assert_eq!(cpu.registers.accumulator, 0x40);
}
