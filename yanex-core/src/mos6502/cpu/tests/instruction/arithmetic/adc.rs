use crate::mos6502::cpu::op_code::OpCode::{
    AdcAbs, AdcAbsX, AdcAbsY, AdcImm, AdcIndX, AdcIndY, AdcZp, AdcZpX,
};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_all, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn flags() {
    let mut cpu = init(data([
        AdcImm as u8,
        0b0111_1111,
        AdcImm as u8,
        0b0111_1111,
        AdcImm as u8,
        0b0111_1111,
        AdcImm as u8,
        0b0111_1111,
    ]));
    cpu.registers.status.d = false;

    cpu.registers.accumulator = 0b0000_0000;
    cpu.registers.status.c = false;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
    assert!(!cpu.registers.status.v);

    cpu.registers.accumulator = 0b0000_0000;
    cpu.registers.status.c = true;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
    assert!(cpu.registers.status.v);

    cpu.registers.accumulator = 0b1000_0000;
    cpu.registers.status.c = false;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
    assert!(cpu.registers.status.v);

    cpu.registers.accumulator = 0b1000_0000;
    cpu.registers.status.c = true;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);
    assert!(!cpu.registers.status.v);
}

#[test]
fn imm() {
    let mut cpu = init(data([AdcImm as u8, 0x69, AdcImm as u8, 0x69]));

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([
            AdcAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            AdcAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
        ]),
        data([0x69]),
    );

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([
            AdcAbsX as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            AdcAbsX as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}

#[test]
fn abs_y() {
    let mut cpu = init_data(
        data([
            AdcAbsY as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            AdcAbsY as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_y = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([
            AdcZp as u8,
            DATA_ZP.to_le_bytes()[0],
            AdcZp as u8,
            DATA_ZP.to_le_bytes()[0],
        ]),
        data([0x69]),
    );

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([
            AdcZpX as u8,
            DATA_ZP.to_le_bytes()[0],
            AdcZpX as u8,
            DATA_ZP.to_le_bytes()[0],
        ]),
        data([0, 0, 0, 0, 0x69]),
    );
    cpu.registers.index_x = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}

#[test]
fn ind_x() {
    let mut cpu = init_all(
        data([
            AdcIndX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
            AdcIndX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
        ]),
        data([0x69]),
        data([DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
    );
    cpu.registers.index_x = 0x04;

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = false;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}

#[test]
fn ind_y() {
    let mut cpu = init_all(
        data([
            AdcIndY as u8,
            DATA_ZP.to_le_bytes()[0],
            AdcIndY as u8,
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
    assert_eq!(cpu.registers.accumulator, 0xAB);

    cpu.registers.accumulator = 0x42;
    cpu.registers.status.c = true;
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0xAC);
}
