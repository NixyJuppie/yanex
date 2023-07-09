use crate::mos6502::cpu::op_code::OpCode::TxsImp;
use crate::mos6502::cpu::tests::tests_helpers::{data, init};

#[test]
fn txs_imp() {
    let mut cpu = init(data([TxsImp as u8]));
    cpu.registers.index_x = 0x69;

    cpu.execute();
    assert_eq!(cpu.registers.stack_pointer, 0x69);
}
