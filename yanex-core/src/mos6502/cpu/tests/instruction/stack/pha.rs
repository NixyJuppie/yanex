use crate::mos6502::cpu::op_code::OpCode::PhaImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn pha_imp() {
    let mut cpu = init(data([PhaImp as u8]));
    cpu.registers.accumulator = 0x69;

    cpu.execute();
    assert_eq!(
        cpu.read_stack(cpu.registers.stack_pointer.wrapping_add(1)),
        0x69
    );
}

#[test]
fn pha_stack_pointer() {
    let mut cpu = init(data([PhaImp as u8, PhaImp as u8]));
    cpu.registers.stack_pointer = 0x00;

    cpu.execute();
    assert_eq!(0xFF, cpu.registers.stack_pointer);
    cpu.execute();
    assert_eq!(0xFE, cpu.registers.stack_pointer);
}
