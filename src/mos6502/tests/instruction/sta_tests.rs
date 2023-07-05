use crate::mos6502::op_code::OpCode::StaAbs;
use crate::mos6502::tests::tests_helpers::{data, init_d, DATA1};

#[test]
fn sta_abs() {
    let mut cpu = init_d(
        data([StaAbs as u8, DATA1.to_le_bytes()[0], DATA1.to_le_bytes()[1]]),
        data([0xFF]),
    );
    cpu.execute();
    assert_eq!(cpu.memory.read(DATA1), 0x00);
}
