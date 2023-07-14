use crate::mos6502::cpu::op_code::OpCode::{DecAbs, DecAbsX, DecZp, DecZpX};
use crate::mos6502::cpu::tests::tests_helpers::{data, init_data, init_data_zp, DATA, DATA_ZP};

#[test]
fn flags() {
    let mut cpu = init_data(
        data([
            DecAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            DecAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
        ]),
        data([0x01, 0x00]),
    );

    cpu.execute();
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.z);

    cpu.execute();
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.z);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([
            DecAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            DecAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
        ]),
        data([0x69, 0x00]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x68);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 1), 0xFF);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([
            DecAbsX as u8,
            (DATA - 4).to_le_bytes()[0],
            (DATA - 4).to_le_bytes()[1],
            DecAbsX as u8,
            (DATA - 3).to_le_bytes()[0],
            (DATA - 3).to_le_bytes()[1],
        ]),
        data([0x69, 0x00]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x68);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 1), 0xFF);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([
            DecZp as u8,
            DATA_ZP.to_le_bytes()[0],
            DecZp as u8,
            (DATA_ZP + 1).to_le_bytes()[0],
        ]),
        data([0x69, 0x00]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0x68);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 1), 0xFF);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([
            DecZpX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
            DecZpX as u8,
            (DATA_ZP - 3).to_le_bytes()[0],
        ]),
        data([0x69, 0x00]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0x68);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 1), 0xFF);
}
