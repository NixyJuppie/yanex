use crate::mos6502::cpu::Cpu;

#[derive(Debug)]
pub enum AddressingMode {
    Implied,
    /// A
    Accumulator,
    /// #$FF
    Immediate,

    /// $FFFF
    Absolute,
    /// $FF
    Relative,
    /// $FF
    ZeroPage,
    /// $FFFF
    Indirect,

    /// $FFFF, X
    AbsoluteIndexedX,
    /// $FFFF, Y
    AbsoluteIndexedY,
    /// $FF, X
    ZeroPageIndexedX,
    /// $FF, Y
    ZeroPageIndexedY,
    /// ($FF, X)
    IndexedIndirectX,
    /// ($FF), Y
    IndirectIndexedY,
}

impl AddressingMode {
    pub fn read(&self, cpu: &mut Cpu) -> u8 {
        match self {
            AddressingMode::Implied => panic!("Cannot read using Implied addressing mode"),
            AddressingMode::Accumulator => cpu.registers.accumulator,
            AddressingMode::Immediate => {
                let data = cpu.memory.read(cpu.registers.program_counter);
                cpu.registers.program_counter += 1;
                data
            }
            AddressingMode::Absolute => {
                let location = cpu.memory.read2(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                cpu.memory.read(location)
            }
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => todo!(),
            AddressingMode::Indirect => todo!(),
            AddressingMode::AbsoluteIndexedX => {
                let location = cpu.memory.read2(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                cpu.memory.read(location + cpu.registers.index_x as u16)
            }
            AddressingMode::AbsoluteIndexedY => {
                let location = cpu.memory.read2(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                cpu.memory.read(location + cpu.registers.index_y as u16)
            }
            AddressingMode::ZeroPageIndexedX => todo!(),
            AddressingMode::ZeroPageIndexedY => todo!(),
            AddressingMode::IndexedIndirectX => todo!(),
            AddressingMode::IndirectIndexedY => todo!(),
        }
    }

    pub fn write(&self, data: u8, cpu: &mut Cpu) {
        match self {
            AddressingMode::Implied => panic!("Cannot read using Implied addressing mode"),
            AddressingMode::Accumulator => panic!("Cannot write using Accumulator addressing mode"),
            AddressingMode::Immediate => panic!("Cannot write using Immediate addressing mode"),
            AddressingMode::Absolute => {
                let location = cpu.memory.read2(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                cpu.memory.write(location, data)
            }
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => todo!(),
            AddressingMode::Indirect => todo!(),
            AddressingMode::AbsoluteIndexedX => todo!(),
            AddressingMode::AbsoluteIndexedY => todo!(),
            AddressingMode::ZeroPageIndexedX => todo!(),
            AddressingMode::ZeroPageIndexedY => todo!(),
            AddressingMode::IndexedIndirectX => todo!(),
            AddressingMode::IndirectIndexedY => todo!(),
        }
    }
}
