use crate::mos6502::cpu::op_code::OpCode::{JmpAbs, JmpInd};
use crate::mos6502::cpu::tests::tests_helpers::{data, init, init_d, DATA1, DATA2};

#[test]
fn jmp_abs() {
    let mut cpu = init(data([JmpAbs as u8, 0x00, 0x00]));

    cpu.execute();
    assert_eq!(cpu.registers.program_counter, 0x00);
}

#[test]
fn jmp_ind() {
    let mut cpu = init_d(
        data([JmpInd as u8, DATA1.to_le_bytes()[0], DATA1.to_le_bytes()[1]]),
        data([DATA2.to_le_bytes()[0], DATA2.to_le_bytes()[1]]),
    );

    cpu.execute();
    assert_eq!(cpu.registers.program_counter, DATA2);
}
