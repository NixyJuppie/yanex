use crate::mos6502::cpu::op_code::OpCode::{
    CmpAbs, CmpAbsX, CmpAbsY, CmpImm, CmpIndX, CmpIndY, CmpZp, CmpZpX,
};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_all, init_data, init_data_zp, DATA, DATA_ZP,
};
use crate::mos6502::cpu::Cpu;

#[test]
fn imm() {
    let mut cpu = init(data([
        CmpImm as u8,
        0b0000_0000,
        CmpImm as u8,
        0b0111_1111,
        CmpImm as u8,
        0b1000_0000,
    ]));
    cpu.registers.accumulator = 0b0111_1111;

    assert_flags(&mut cpu);
}

#[test]
fn abs() {
    let mut cpu = init_data(
        data([
            CmpAbs as u8,
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            CmpAbs as u8,
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
            CmpAbs as u8,
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.accumulator = 0b0111_1111;

    assert_flags(&mut cpu);
}

#[test]
fn abs_x() {
    let mut cpu = init_data(
        data([
            CmpAbsX as u8,
            (DATA - 4).to_le_bytes()[0],
            (DATA - 4).to_le_bytes()[1],
            CmpAbsX as u8,
            (DATA - 3).to_le_bytes()[0],
            (DATA - 3).to_le_bytes()[1],
            CmpAbsX as u8,
            (DATA - 2).to_le_bytes()[0],
            (DATA - 2).to_le_bytes()[1],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.accumulator = 0b0111_1111;
    cpu.registers.index_x = 0x04;

    assert_flags(&mut cpu);
}

#[test]
fn abs_y() {
    let mut cpu = init_data(
        data([
            CmpAbsY as u8,
            (DATA - 4).to_le_bytes()[0],
            (DATA - 4).to_le_bytes()[1],
            CmpAbsY as u8,
            (DATA - 3).to_le_bytes()[0],
            (DATA - 3).to_le_bytes()[1],
            CmpAbsY as u8,
            (DATA - 2).to_le_bytes()[0],
            (DATA - 2).to_le_bytes()[1],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.accumulator = 0b0111_1111;
    cpu.registers.index_y = 0x04;

    assert_flags(&mut cpu);
}

#[test]
fn zp() {
    let mut cpu = init_data_zp(
        data([
            CmpZp as u8,
            DATA_ZP.to_le_bytes()[0],
            CmpZp as u8,
            (DATA_ZP + 1).to_le_bytes()[0],
            CmpZp as u8,
            (DATA_ZP + 2).to_le_bytes()[0],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.accumulator = 0b0111_1111;

    assert_flags(&mut cpu);
}

#[test]
fn zp_x() {
    let mut cpu = init_data_zp(
        data([
            CmpZpX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
            CmpZpX as u8,
            (DATA_ZP - 3).to_le_bytes()[0],
            CmpZpX as u8,
            (DATA_ZP - 2).to_le_bytes()[0],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
    );
    cpu.registers.accumulator = 0b0111_1111;
    cpu.registers.index_x = 0x04;

    assert_flags(&mut cpu);
}

#[test]
fn ind_x() {
    let mut cpu = init_all(
        data([
            CmpIndX as u8,
            (DATA_ZP - 4).to_le_bytes()[0],
            CmpIndX as u8,
            (DATA_ZP - 2).to_le_bytes()[0],
            CmpIndX as u8,
            DATA_ZP.to_le_bytes()[0],
        ]),
        data([0b0000_0000, 0b0111_1111, 0b1000_0000]),
        data([
            DATA.to_le_bytes()[0],
            DATA.to_le_bytes()[1],
            (DATA + 1).to_le_bytes()[0],
            (DATA + 1).to_le_bytes()[1],
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
        ]),
    );
    cpu.registers.accumulator = 0b0111_1111;
    cpu.registers.index_x = 0x04;

    assert_flags(&mut cpu);
}

#[test]
fn ind_y() {
    let mut cpu = init_all(
        data([
            CmpIndY as u8,
            DATA_ZP.to_le_bytes()[0],
            CmpIndY as u8,
            (DATA_ZP + 2).to_le_bytes()[0],
            CmpIndY as u8,
            (DATA_ZP + 4).to_le_bytes()[0],
        ]),
        data([
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
            0b0000_0000,
            (DATA + 5).to_le_bytes()[0],
            (DATA + 5).to_le_bytes()[1],
            0b0111_1111,
            (DATA + 8).to_le_bytes()[0],
            (DATA + 8).to_le_bytes()[1],
            0b1000_0000,
        ]),
        data([
            (DATA - 4).to_le_bytes()[0],
            (DATA - 4).to_le_bytes()[1],
            (DATA - 1).to_le_bytes()[0],
            (DATA - 1).to_le_bytes()[1],
            (DATA + 2).to_le_bytes()[0],
            (DATA + 2).to_le_bytes()[1],
        ]),
    );
    cpu.registers.accumulator = 0b0111_1111;
    cpu.registers.index_y = 0x04;

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
