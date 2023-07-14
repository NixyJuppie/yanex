use crate::mos6502::cpu::op_code::OpCode::{BitAbs, BitZp};
use crate::mos6502::cpu::tests::tests_helpers::{data, init_data, init_data_zp, DATA, DATA_ZP};

#[test]
fn abs() {
    let mut cpu = init_data(
        data([
            BitAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            BitAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
        ]),
        data([0x00, 0xFF]),
    );
    cpu.registers.accumulator = 0b1111_1111;

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(!cpu.registers.status.v);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(cpu.registers.status.v);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([
            BitZp as u8,
            DATA_ZP.to_le_bytes()[0],
            BitZp as u8,
            (DATA_ZP + 1).to_le_bytes()[0],
        ]),
        data([0x00, 0xFF]),
    );
    cpu.registers.accumulator = 0b1111_1111;

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(!cpu.registers.status.v);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(cpu.registers.status.v);
}
