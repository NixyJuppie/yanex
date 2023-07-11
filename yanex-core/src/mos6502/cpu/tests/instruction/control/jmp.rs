use crate::mos6502::cpu::op_code::OpCode::{JmpAbs, JmpInd};
use crate::mos6502::cpu::tests::tests_helpers::{data, init, init_data, DATA, DATA_ZP};

#[test]
fn jmp_abs() {
    let mut cpu = init(data([JmpAbs as u8, 0x00, 0x00]));

    cpu.execute();
    assert_eq!(cpu.registers.program_counter, 0x00);
}

#[test]
fn jmp_ind() {
    let mut cpu = init_data(
        data([JmpInd as u8, DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
        data([DATA_ZP.to_le_bytes()[0], DATA_ZP.to_le_bytes()[1]]),
    );

    cpu.execute();
    assert_eq!(cpu.registers.program_counter, DATA_ZP);
}
