use crate::mos6502::cpu::op_code::OpCode::PhpImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn php_imp() {
    let mut cpu = init(data([PhpImp as u8]));
    cpu.registers.status.c = true;
    cpu.registers.status.z = true;
    cpu.registers.status.i = true;
    cpu.registers.status.d = true;
    cpu.registers.status.b = true;
    cpu.registers.status.u = true;
    cpu.registers.status.v = true;
    cpu.registers.status.n = true;

    cpu.execute();
    assert_eq!(
        cpu.registers.status,
        cpu.read_stack(cpu.registers.stack_pointer).into()
    );
}
