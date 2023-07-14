use crate::mos6502::cpu::op_code::OpCode::DeyImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn flags() {
    let mut cpu = init(data([DeyImp as u8, DeyImp as u8]));

    cpu.registers.index_y = 0x01;
    cpu.execute();
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.z);

    cpu.registers.index_y = 0x00;
    cpu.execute();
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.z);
}

#[test]
fn imm() {
    let mut cpu = init(data([DeyImp as u8, DeyImp as u8]));

    cpu.registers.index_y = 0x69;
    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0x68);

    cpu.registers.index_y = 0x00;
    cpu.execute();
    assert_eq!(cpu.registers.index_y, 0xFF);
}
