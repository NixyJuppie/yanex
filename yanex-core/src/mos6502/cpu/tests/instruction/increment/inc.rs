use crate::mos6502::cpu::op_code::OpCode::{IncAbs, IncAbsX, IncZp, IncZpX};
use crate::mos6502::cpu::tests::tests_helpers::{data, init_data, init_data_zp, DATA, DATA_ZP};

#[test]
fn flags() {
    let mut cpu = init_data(
        data([
            IncAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            IncAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
        ]),
        data([0xFE, 0xFF]),
    );

    cpu.execute();
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.z);

    cpu.execute();
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.z);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([
            IncAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            IncAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
        ]),
        data([0x69, 0xFF]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x6A);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 1), 0x00);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([
            IncAbsX as u8,
            (DATA - 4).to_le_bytes()[0],
            (DATA - 4).to_le_bytes()[1],
            IncAbsX as u8,
            (DATA - 3).to_le_bytes()[0],
            (DATA - 3).to_le_bytes()[1],
        ]),
        data([0x69, 0xFF]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x6A);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 1), 0x00);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([
            IncZp as u8,
            DATA_ZP.to_le_bytes()[0],
            IncZp as u8,
            (DATA_ZP + 1).to_le_bytes()[0],
        ]),
        data([0x69, 0xFF]),
    );

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0x6A);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 1), 0x00);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([
            IncZpX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
            IncZpX as u8,
            (DATA_ZP - 3).to_le_bytes()[0],
        ]),
        data([0x69, 0xFF]),
    );
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0x6A);

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 1), 0x00);
}
