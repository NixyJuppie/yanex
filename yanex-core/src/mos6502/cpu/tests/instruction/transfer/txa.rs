use crate::mos6502::cpu::op_code::OpCode::TxaImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn flags() {
    let mut cpu = init(data([TxaImp as u8, TxaImp as u8]));

    cpu.registers.index_x = 0x00;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.registers.index_x = 0xFF;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn imp() {
    let mut cpu = init(data([TxaImp as u8]));
    cpu.registers.index_x = 0x69;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}
