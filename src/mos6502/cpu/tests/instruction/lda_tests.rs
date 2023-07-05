use crate::mos6502::cpu::op_code::OpCode::{LdaAbs, LdaImm};
use crate::mos6502::cpu::tests::tests_helpers::{data, init, init_d, DATA1};

#[test]
fn lda_imm() {
    let mut cpu = init(data([LdaImm as u8, 0x10]));
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x10);
}

#[test]
fn lda_abs() {
    let mut cpu = init_d(
        data([LdaAbs as u8, DATA1.to_le_bytes()[0], DATA1.to_le_bytes()[1]]),
        data([0x12]),
    );
    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x12);
}
