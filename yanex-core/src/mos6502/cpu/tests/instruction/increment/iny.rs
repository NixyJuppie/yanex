use crate::mos6502::cpu::op_code::OpCode::InyImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn flags() {
    let mut cpu = init(data([InyImp as u8, InyImp as u8]));

    cpu.registers.index_y = 0xFE;
    cpu.execute();
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.z);

    cpu.registers.index_y = 0xFF;
    cpu.execute();
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.z);
}

#[test]
fn imm() {
    let mut cpu = init(data([InyImp as u8, InyImp as u8]));

    cpu.registers.index_y = 0x69;
    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x6A);

    cpu.registers.index_y = 0xFF;
    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x00);
}
