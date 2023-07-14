use crate::mos6502::cpu::op_code::OpCode::PlaImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn imp() {
    let mut cpu = init(data([PlaImp as u8]));
    cpu.write_stack(cpu.registers.stack_pointer.wrapping_add(1), 0x69);

    cpu.execute();
    assert_eq!(cpu.registers.accumulator, 0x69);
}

#[test]
fn stack_pointer() {
    let mut cpu = init(data([PlaImp as u8, PlaImp as u8]));
    cpu.registers.stack_pointer = 0xFF;

    cpu.execute();
    assert_eq!(0x00, cpu.registers.stack_pointer);
    cpu.execute();
    assert_eq!(0x01, cpu.registers.stack_pointer);
}
