use crate::mos6502::cpu::op_code::OpCode::{
    SbcAbs, SbcAbsX, SbcAbsY, SbcImm, SbcIndX, SbcIndY, SbcZp, SbcZpX,
};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_all, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn flags() {
    let mut cpu = init(data([
        SbcImm as u8,
        0b0111_1111,
        SbcImm as u8,
        0b0111_1111,
        SbcImm as u8,
        0b0111_1111,
        SbcImm as u8,
        0b0111_1111,
    ]));
    cpu.registers.status.d = false;

    cpu.registers.accumulator = 0b0000_0000;
    cpu.registers.status.c = false;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
    assert!(!cpu.registers.status.v);

    cpu.registers.accumulator = 0b0000_0000;
    cpu.registers.status.c = true;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
    assert!(!cpu.registers.status.v);

    cpu.registers.accumulator = 0b1000_0000;
    cpu.registers.status.c = false;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);
    assert!(cpu.registers.status.v);

    cpu.registers.accumulator = 0b1000_0000;
    cpu.registers.status.c = true;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);
    assert!(cpu.registers.status.v);
}

#[test]
fn imm() {
    let mut cpu = init(data([SbcImm as u8, 0x69, SbcImm as u8, 0x69]));

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([
            SbcAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            SbcAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
        ]),
        data([0x69]),
    );

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([
            SbcAbsX as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            SbcAbsX as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}

#[test]
fn abs_y() {
    let mut cpu = init_data(
        data([
            SbcAbsY as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            SbcAbsY as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_y = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([
            SbcZp as u8,
            DATA_ZP.to_le_bytes()[0],
            SbcZp as u8,
            DATA_ZP.to_le_bytes()[0],
        ]),
        data([0x69]),
    );

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([
            SbcZpX as u8,
            DATA_ZP.to_le_bytes()[0],
            SbcZpX as u8,
            DATA_ZP.to_le_bytes()[0],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}

#[test]
fn ind_x() {
    let mut cpu = init_all(
        data([
            SbcIndX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
            SbcIndX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
        ]),
        data([0x69]),
        data([DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
    );
    cpu.registers.index_x = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}

#[test]
fn ind_y() {
    let mut cpu = init_all(
        data([
            SbcIndY as u8,
            DATA_ZP.to_le_bytes()[0],
            SbcIndY as u8,
            DATA_ZP.to_le_bytes()[0],
        ]),
        data([
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
            0x69,
        ]),
        data([(DATA - 4).to_le_bytes()[0], (DATA - 4).to_le_bytes()[1]]),
    );
    cpu.registers.index_y = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD8);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xD9);
}
