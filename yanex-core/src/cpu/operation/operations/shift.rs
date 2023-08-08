use super::{mem_write, AddressingMode, AddressingModeReadData, AddressingModeWriteData};
use crate::cpu::{Cpu, CpuMemory, CpuStatus};

#[derive(Debug, Clone)]
pub enum ShiftRight {
    Decoded(AddressingMode),
    ReadingData(AddressingMode, AddressingModeReadData),
    WritingData(AddressingModeWriteData),
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

                let mut read = mode.begin_read_data();
                match read.read(cpu, memory) {
                    None => {
                        *self = ShiftRight::ReadingData(mode.clone(), read);
                        None
                    }
                    Some(data) => {
                        let result = shift_right(data, &mut cpu.registers.status);
                        let write = mode.begin_write_data(result);
                        *self = ShiftRight::WritingData(write);
                        None
                    }
                }
            }
            ShiftRight::ReadingData(mode, read) => {
                if let Some(data) = read.read(cpu, memory) {
                    let result = shift_right(data, &mut cpu.registers.status);
                    let write = mode.begin_write_data(result);
                    *self = ShiftRight::WritingData(write);
                }
                None
            }
            ShiftRight::WritingData(write) => {
                let mut write = write.clone();
                mem_write!(self, cpu, memory, write, WritingData)?;
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
    ReadingData(AddressingMode, AddressingModeReadData),
    WritingData(AddressingModeWriteData),
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

                let mut read = mode.begin_read_data();
                match read.read(cpu, memory) {
                    None => {
                        *self = ShiftLeft::ReadingData(mode.clone(), read);
                        None
                    }
                    Some(data) => {
                        let result = shift_left(data, &mut cpu.registers.status);
                        let write = mode.begin_write_data(result);
                        *self = ShiftLeft::WritingData(write);
                        None
                    }
                }
            }
            ShiftLeft::ReadingData(mode, read) => {
                if let Some(data) = read.read(cpu, memory) {
                    let result = shift_left(data, &mut cpu.registers.status);
                    let write = mode.begin_write_data(result);
                    *self = ShiftLeft::WritingData(write);
                }
                None
            }
            ShiftLeft::WritingData(write) => {
                let mut write = write.clone();
                mem_write!(self, cpu, memory, write, WritingData)?;
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
    ReadingData(AddressingMode, AddressingModeReadData),
    WritingData(AddressingModeWriteData),
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

                let mut read = mode.begin_read_data();
                match read.read(cpu, memory) {
                    None => {
                        *self = RotateLeft::ReadingData(mode.clone(), read);
                        None
                    }
                    Some(data) => {
                        let result = rotate_left(data, &mut cpu.registers.status);
                        let write = mode.begin_write_data(result);
                        *self = RotateLeft::WritingData(write);
                        None
                    }
                }
            }
            RotateLeft::ReadingData(mode, read) => {
                if let Some(data) = read.read(cpu, memory) {
                    let result = rotate_left(data, &mut cpu.registers.status);
                    let write = mode.begin_write_data(result);
                    *self = RotateLeft::WritingData(write);
                }
                None
            }
            RotateLeft::WritingData(write) => {
                let mut write = write.clone();
                mem_write!(self, cpu, memory, write, WritingData)?;
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
    ReadingData(AddressingMode, AddressingModeReadData),
    WritingData(AddressingModeWriteData),
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

                let mut read = mode.begin_read_data();
                match read.read(cpu, memory) {
                    None => {
                        *self = RotateRight::ReadingData(mode.clone(), read);
                        None
                    }
                    Some(data) => {
                        let result = rotate_right(data, &mut cpu.registers.status);
                        let write = mode.begin_write_data(result);
                        *self = RotateRight::WritingData(write);
                        None
                    }
                }
            }
            RotateRight::ReadingData(mode, read) => {
                if let Some(data) = read.read(cpu, memory) {
                    let result = rotate_right(data, &mut cpu.registers.status);
                    let write = mode.begin_write_data(result);
                    *self = RotateRight::WritingData(write);
                }
                None
            }
            RotateRight::WritingData(write) => {
                let mut write = write.clone();
                mem_write!(self, cpu, memory, write, WritingData)?;
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
