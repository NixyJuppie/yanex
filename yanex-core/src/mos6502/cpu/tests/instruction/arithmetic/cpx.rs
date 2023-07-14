use crate::mos6502::cpu::op_code::OpCode::{CpxAbs, CpxImm, CpxZp};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};
use crate::mos6502::cpu::Cpu;

#[test]
fn imm() {
    let mut cpu = init(data([
        CpxImm as u8,
        0b0000_0000,
        CpxImm as u8,
        0b0111_1111,
        CpxImm as u8,
        0b1000_0000,
    ]));
    cpu.registers.index_x = 0b0111_1111;

    assert_flags(&mut cpu);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([
            CpxAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            CpxAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
            CpxAbs as u8,
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.index_x = 0b0111_1111;

    assert_flags(&mut cpu);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([
            CpxZp as u8,
            DATA_ZP.to_le_bytes()[0],
            CpxZp as u8,
            (DATA_ZP + 1).to_le_bytes()[0],
            CpxZp as u8,
            (DATA_ZP + 2).to_le_bytes()[0],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.index_x = 0b0111_1111;

    assert_flags(&mut cpu);
}

fn assert_flags(cpu: &mut Cpu) {
    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);

    cpu.execute();
    assert!(cpu.registers.status.z);
    assert!(!cpu.registers.status.n);
    assert!(cpu.registers.status.c);

    cpu.execute();
    assert!(!cpu.registers.status.z);
    assert!(cpu.registers.status.n);
    assert!(!cpu.registers.status.c);
}
