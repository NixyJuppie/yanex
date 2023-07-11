use crate::mos6502::cpu::op_code::OpCode::PlpImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn plp_imp() {
    let mut cpu = init(data([PlpImp as u8]));
    cpu.write_stack(cpu.registers.stack_pointer, 0xFF);

    cpu.execute();
    assert_eq!(cpu.registers.status, 0xFF.into());
}
