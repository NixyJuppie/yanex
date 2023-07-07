use crate::mos6502::cpu::op_code::OpCode::NopImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn nop_imp() {
    let mut cpu = init(data([NopImp as u8]));
    let cpu_backup = cpu.clone();

    cpu.execute();
    assert_ne!(
        cpu_backup.registers.program_counter,
        cpu.registers.program_counter
    );
    cpu.registers.program_counter = cpu_backup.registers.program_counter;
    assert_eq!(cpu_backup, cpu);
}
