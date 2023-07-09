use crate::mos6502::cpu::op_code::OpCode::TyaImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn tya_flags() {
    let mut cpu = init(data([TyaImp as u8, TyaImp as u8]));

    cpu.registers.index_y = 0x00;
    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);

    cpu.registers.index_y = 0xFF;
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
}

#[test]
fn tya_imp() {
    let mut cpu = init(data([TyaImp as u8]));
    cpu.registers.index_y = 0x69;

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}
