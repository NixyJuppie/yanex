use crate::mos6502::cpu::op_code::OpCode::TaxImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn tax_flags() {
    let mut cpu = init(data([TaxImp as u8, TaxImp as u8]));

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
fn tax_imp() {
    let mut cpu = init(data([TaxImp as u8]));
    cpu.registers.accumulator = 0x69;

    cpu.execute();
    assert_eq!(cpu.registers.index_x, 0x69);
}
