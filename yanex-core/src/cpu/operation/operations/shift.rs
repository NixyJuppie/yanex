use super::{mem_read, AddressingMode, ReadAddress};
use crate::cpu::{Cpu, CpuMemory, CpuStatus};

#[derive(Debug, Clone)]
pub enum ShiftRight {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    ReadAddress(u16),
    DummyRead(u16),
    ReadData(u8, u16),
}

impl ShiftRight {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ShiftRight::Decoded(mode) => {
                if *mode == AddressingMode::Accumulator {
                    cpu.registers.accumulator =
                        shift_right(cpu.registers.accumulator, &mut cpu.registers.status);
                    return Some(());
                }

                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = ShiftRight::ReadAddress(address);
                None
            }
            ShiftRight::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = ShiftRight::ReadAddress(address);
                None
            }
            ShiftRight::ReadAddress(address) => {
                *self = ShiftRight::DummyRead(*address);
                None
            }
            ShiftRight::DummyRead(address) => {
                let data = memory.read_u8(*address);

                *self = ShiftRight::ReadData(data, *address);
                None
            }
            ShiftRight::ReadData(data, address) => {
                let data = shift_right(*data, &mut cpu.registers.status);
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

fn shift_right(input: u8, status: &mut CpuStatus) -> u8 {
    let result = input >> 1;
    status.set_negative(false);
    status.set_zero(result == 0);
    status.set_carry(input & 0b_0000_0001 > 0);
    result
}

#[derive(Debug, Clone)]
pub enum ShiftLeft {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    ReadAddress(u16),
    DummyRead(u16),
    ReadData(u8, u16),
}

impl ShiftLeft {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            ShiftLeft::Decoded(mode) => {
                if *mode == AddressingMode::Accumulator {
                    cpu.registers.accumulator =
                        shift_left(cpu.registers.accumulator, &mut cpu.registers.status);
                    return Some(());
                }

                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = ShiftLeft::ReadAddress(address);
                None
            }
            ShiftLeft::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = ShiftLeft::ReadAddress(address);
                None
            }
            ShiftLeft::ReadAddress(address) => {
                *self = ShiftLeft::DummyRead(*address);
                None
            }
            ShiftLeft::DummyRead(address) => {
                let data = memory.read_u8(*address);

                *self = ShiftLeft::ReadData(data, *address);
                None
            }
            ShiftLeft::ReadData(data, address) => {
                let data = shift_left(*data, &mut cpu.registers.status);
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

fn shift_left(input: u8, status: &mut CpuStatus) -> u8 {
    let result = input << 1;
    status.set_negative(result & 0b_1000_0000 > 0);
    status.set_zero(result == 0);
    status.set_carry(input & 0b_1000_0000 > 0);
    result
}

#[derive(Debug, Clone)]
pub enum RotateLeft {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    ReadAddress(u16),
    DummyRead(u16),
    ReadData(u8, u16),
}

impl RotateLeft {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            RotateLeft::Decoded(mode) => {
                if *mode == AddressingMode::Accumulator {
                    cpu.registers.accumulator =
                        rotate_left(cpu.registers.accumulator, &mut cpu.registers.status);
                    return Some(());
                }

                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = RotateLeft::ReadAddress(address);
                None
            }
            RotateLeft::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = RotateLeft::ReadAddress(address);
                None
            }
            RotateLeft::ReadAddress(address) => {
                *self = RotateLeft::DummyRead(*address);
                None
            }
            RotateLeft::DummyRead(address) => {
                let data = memory.read_u8(*address);

                *self = RotateLeft::ReadData(data, *address);
                None
            }
            RotateLeft::ReadData(data, address) => {
                let data = rotate_left(*data, &mut cpu.registers.status);
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

fn rotate_left(input: u8, status: &mut CpuStatus) -> u8 {
    let mut result = input << 1;
    if status.carry() {
        result |= 0b_0000_0001;
    } else {
        result &= 0b_1111_1110;
    }

    status.set_negative(input & 0b_0100_0000 > 0);
    status.set_zero(result == 0);
    status.set_carry(input & 0b_1000_0000 > 0);
    result
}

#[derive(Debug, Clone)]
pub enum RotateRight {
    Decoded(AddressingMode),
    ReadingAddress(ReadAddress),
    ReadAddress(u16),
    DummyRead(u16),
    ReadData(u8, u16),
}

impl RotateRight {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            RotateRight::Decoded(mode) => {
                if *mode == AddressingMode::Accumulator {
                    cpu.registers.accumulator =
                        rotate_right(cpu.registers.accumulator, &mut cpu.registers.status);
                    return Some(());
                }

                let mut read = mode.begin_read_address();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = RotateRight::ReadAddress(address);
                None
            }
            RotateRight::ReadingAddress(read) => {
                let mut read = read.clone();
                let address = mem_read!(self, cpu, memory, read, ReadingAddress)?;

                *self = RotateRight::ReadAddress(address);
                None
            }
            RotateRight::ReadAddress(address) => {
                *self = RotateRight::DummyRead(*address);
                None
            }
            RotateRight::DummyRead(address) => {
                let data = memory.read_u8(*address);

                *self = RotateRight::ReadData(data, *address);
                None
            }
            RotateRight::ReadData(data, address) => {
                let data = rotate_right(*data, &mut cpu.registers.status);
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

fn rotate_right(input: u8, status: &mut CpuStatus) -> u8 {
    let mut result = input >> 1;
    if status.carry() {
        result |= 0b_1000_0000;
    } else {
        result &= 0b_0111_1111;
    }

    status.set_negative(status.carry());
    status.set_zero(result == 0);
    status.set_carry(input & 0b_0000_0001 > 0);
    result
}
