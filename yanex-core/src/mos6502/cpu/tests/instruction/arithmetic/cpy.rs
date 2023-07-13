use crate::mos6502::cpu::op_code::OpCode::{CpyAbs, CpyImm, CpyZp};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_data, init_data_zp, DATA, DATA_ZP,
};
use crate::mos6502::cpu::Cpu;

#[test]
fn cpy_imm() {
    let mut cpu = init(data([
        CpyImm as u8,
        0b0000_0000,
        CpyImm as u8,
        0b0111_1111,
        CpyImm as u8,
        0b1000_0000,
    ]));
    cpu.registers.index_y = 0b0111_1111;

    assert_flags(&mut cpu);
}

#[test]
fn cpy_abs() {
    let mut cpu = init_data(
        data([
            CpyAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            CpyAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
            CpyAbs as u8,
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.index_y = 0b0111_1111;

    assert_flags(&mut cpu);
}

#[test]
fn cpy_zp() {
    let mut cpu = init_data_zp(
        data([
            CpyZp as u8,
            DATA_ZP.to_le_bytes()[0],
            CpyZp as u8,
            (DATA_ZP + 1).to_le_bytes()[0],
            CpyZp as u8,
            (DATA_ZP + 2).to_le_bytes()[0],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.index_y = 0b0111_1111;

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
