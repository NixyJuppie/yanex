use crate::mos6502::cpu::op_code::OpCode::{
    StaAbs, StaAbsX, StaAbsY, StaIndX, StaIndY, StaZp, StaZpX,
};
use crate::mos6502::cpu::tests::tests_helpers::{
    data, init, init_all, init_data, init_data_zp, DATA, DATA_ZP,
};

#[test]
fn sta_abs() {
    let mut cpu = init(data([
        StaAbs as u8,
        DATA.to_le_bytes()[0],
        DATA.to_le_bytes()[1],
    ]));
    cpu.registers.accumulator = 0x69;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x69);
}

#[test]
fn sta_abs_x() {
    let mut cpu = init(data([
        StaAbsX as u8,
        DATA.to_le_bytes()[0],
        DATA.to_le_bytes()[1],
    ]));
    cpu.registers.accumulator = 0x69;
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 4), 0x69);
}

#[test]
fn sta_abs_y() {
    let mut cpu = init(data([
        StaAbsY as u8,
        DATA.to_le_bytes()[0],
        DATA.to_le_bytes()[1],
    ]));
    cpu.registers.accumulator = 0x69;
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA + 4), 0x69);
}

#[test]
fn sta_zp() {
    let mut cpu = init(data([StaZp as u8, DATA_ZP.to_le_bytes()[0]]));
    cpu.registers.accumulator = 0x69;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP), 0x69);
}

#[test]
fn sta_zp_x() {
    let mut cpu = init(data([StaZpX as u8, DATA_ZP.to_le_bytes()[0]]));
    cpu.registers.accumulator = 0x69;
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA_ZP + 4), 0x69);
}

#[test]
fn sta_ind_x() {
    let mut cpu = init_data_zp(
        data([StaIndX as u8, (DATA_ZP - 4).to_le_bytes()[0]]),
        data([DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
    );
    cpu.registers.accumulator = 0x69;
    cpu.registers.index_x = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x69);
}

#[test]
fn sta_ind_y() {
    let mut cpu = init_data_zp(
        data([StaIndY as u8, (DATA_ZP - 4).to_le_bytes()[0]]),
        data([DATA.to_le_bytes()[0], DATA.to_le_bytes()[1]]),
    );
    cpu.registers.accumulator = 0x69;
    cpu.registers.index_y = 0x04;

    cpu.execute();
    assert_eq!(cpu.memory.read_u8(DATA), 0x69);
}
