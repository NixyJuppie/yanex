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
    pub fn read_address(&self, cpu: &mut Cpu) -> u16 {
        use crate::mos6502::cpu::addressing_mode::AddressingMode::*;

        match self {
            Implied => panic!("Cannot read address using Implied addressing mode"),
            Accumulator => panic!("Cannot read address using Accumulator addressing mode"),
            Immediate => panic!("Cannot read address using Immediate addressing mode"),
            Absolute => {
                let location = cpu.memory.read_u16(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                location
            }
            Relative => todo!(),
            ZeroPage => todo!(),
            Indirect => {
                let location = cpu.memory.read_u16(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                cpu.memory.read_u16(location)
            }
            AbsoluteIndexedX => {
                let location = cpu.memory.read_u16(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                location + cpu.registers.index_x as u16
            }
            AbsoluteIndexedY => {
                let location = cpu.memory.read_u16(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                location + cpu.registers.index_y as u16
            }
            ZeroPageIndexedX => todo!(),
            ZeroPageIndexedY => todo!(),
            IndexedIndirectX => todo!(),
            IndirectIndexedY => todo!(),
        }
    }

    pub fn read_data(&self, cpu: &mut Cpu) -> u8 {
        use crate::mos6502::cpu::addressing_mode::AddressingMode::*;

        match self {
            Implied => panic!("Cannot read data using Implied addressing mode"),
            Accumulator => cpu.registers.accumulator,
            Immediate => {
                let data = cpu.memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter += 1;
                data
            }
            Absolute => {
                let location = Absolute.read_address(cpu);
                cpu.memory.read_u8(location)
            }
            Relative => todo!(),
            ZeroPage => todo!(),
            Indirect => todo!(),
            AbsoluteIndexedX => {
                let location = AbsoluteIndexedX.read_address(cpu);
                cpu.memory.read_u8(location)
            }
            AbsoluteIndexedY => {
                let location = AbsoluteIndexedY.read_address(cpu);
                cpu.memory.read_u8(location)
            }
            ZeroPageIndexedX => todo!(),
            ZeroPageIndexedY => todo!(),
            IndexedIndirectX => todo!(),
            IndirectIndexedY => todo!(),
        }
    }

    pub fn write(&self, data: u8, cpu: &mut Cpu) {
        use crate::mos6502::cpu::addressing_mode::AddressingMode::*;

        match self {
            Implied => panic!("Cannot write using Implied addressing mode"),
            Accumulator => panic!("Cannot write using Accumulator addressing mode"),
            Immediate => panic!("Cannot write using Immediate addressing mode"),
            Absolute => {
                let location = cpu.memory.read_u16(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                cpu.memory.write_u8(location, data)
            }
            Relative => todo!(),
            ZeroPage => todo!(),
            Indirect => todo!(),
            AbsoluteIndexedX => todo!(),
            AbsoluteIndexedY => todo!(),
            ZeroPageIndexedX => todo!(),
            ZeroPageIndexedY => todo!(),
            IndexedIndirectX => todo!(),
            IndirectIndexedY => todo!(),
        }
    }
}
