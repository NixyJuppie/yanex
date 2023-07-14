use crate::mos6502::cpu::op_code::OpCode::TayImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn flags() {
    let mut cpu = init(data([TayImp as u8, TayImp as u8]));

    cpu.registers.accumulator = 0x00;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.registers.accumulator = 0xFF;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn imp() {
    let mut cpu = init(data([TayImp as u8]));
    cpu.registers.accumulator = 0x69;

    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x69);
}
