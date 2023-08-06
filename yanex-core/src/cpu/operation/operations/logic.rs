use super::{mem_read, AddressingModeReadData};
use crate::cpu::{AddressingMode, Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum BitTest {
    Decoded(AddressingMode),
    ReadingData(AddressingModeReadData),
}

impl BitTest {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            BitTest::Decoded(mode) => {
                let mut read = mode.begin_read_data();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator & data;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_overflow(data & 0b_0100_0000 > 0);
                cpu.registers.status.set_negative(data & 0b_1000_0000 > 0);
                Some(())
            }
            BitTest::ReadingData(read) => {
                let mut read = read.clone();
                let data = mem_read!(self, cpu, memory, read, ReadingData)?;

                let result = cpu.registers.accumulator & data;
                cpu.registers.status.set_zero(result == 0);
                cpu.registers.status.set_overflow(data & 0b_0100_0000 > 0);
                cpu.registers.status.set_negative(data & 0b_1000_0000 > 0);
                Some(())
            }
        }
    }
}
