use crate::mos6502::cpu::op_code::OpCode::TsxImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn flags() {
    let mut cpu = init(data([TsxImp as u8, TsxImp as u8]));

    cpu.registers.stack_pointer = 0x00;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.registers.stack_pointer = 0xFF;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn imp() {
    let mut cpu = init(data([TsxImp as u8]));
    cpu.registers.stack_pointer = 0x69;

    cpu.execute();
    assert_eq!(cpu.registers.index_x, 0x69);
}
