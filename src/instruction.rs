use crate::cpu::Cpu;

pub fn execute_instruction(op_code: u8, cpu: &mut Cpu) {
    match op_code {
        0xEA => (), // NOP
        0xA9 => lda(AddressingMode::Immediate, cpu),
        0xAD => lda(AddressingMode::Absolute, cpu),
        0xBD => lda(AddressingMode::XIndexedAbsolute, cpu),
        0xB9 => lda(AddressingMode::YIndexedAbsolute, cpu),
        0xA5 => lda(AddressingMode::ZeroPage, cpu),
        0xB5 => lda(AddressingMode::XIndexedZeroPage, cpu),
        0xA1 => lda(AddressingMode::XIndexedZeroPageIndirect, cpu),
        0xB1 => lda(AddressingMode::ZeroPageIndirectYIndexed, cpu),
        0x8D => sta(AddressingMode::Absolute, cpu),
        0x9D => sta(AddressingMode::XIndexedAbsolute, cpu),
        0x99 => sta(AddressingMode::YIndexedAbsolute, cpu),
        0x85 => sta(AddressingMode::ZeroPage, cpu),
        0x95 => sta(AddressingMode::XIndexedZeroPage, cpu),
        0x81 => sta(AddressingMode::XIndexedZeroPageIndirect, cpu),
        0x91 => sta(AddressingMode::ZeroPageIndirectYIndexed, cpu),
        _ => unimplemented!("OpCode: 0x{op_code:2x}"),
    }
}

fn lda(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = addressing_mode.read(cpu);
    cpu.registers.accumulator = data;
    cpu.registers.status.z = data == 0;
    cpu.registers.status.n = data & 0b10000000 == 0b10000000;
}

fn sta(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = cpu.registers.accumulator;
    addressing_mode.write(data, cpu);
}

#[derive(Debug)]
enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    XIndexedAbsolute,
    YIndexedAbsolute,
    ZeroPage,
    XIndexedZeroPage,
    XIndexedZeroPageIndirect,
    ZeroPageIndirectYIndexed,
}

impl AddressingMode {
    fn read(&self, cpu: &mut Cpu) -> u8 {
        match self {
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
            _ => unimplemented!("AddressingMode: {:?}", self),
        }
    }

    fn write(&self, data: u8, cpu: &mut Cpu) {
        match self {
            AddressingMode::Absolute => {
                let location = cpu.memory.read2(cpu.registers.program_counter);
                cpu.registers.program_counter += 2;
                cpu.memory.write(location, data)
            }
            _ => unimplemented!("AddressingMode: {:?}", self),
        }
    }
}
