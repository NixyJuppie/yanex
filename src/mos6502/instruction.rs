use crate::mos6502::addressing_mode::AddressingMode;
use crate::mos6502::cpu::Cpu;
use crate::mos6502::op_code::OpCode;

pub fn execute_instruction(op_code: OpCode, cpu: &mut Cpu) {
    use crate::mos6502::addressing_mode::AddressingMode::*;
    use crate::mos6502::op_code::OpCode::*;

    match op_code {
        LdaImm => lda(Immediate, cpu),
        LdaAbs => lda(Absolute, cpu),
        LdaAbsX => lda(AbsoluteIndexedX, cpu),
        LdaAbsY => lda(AbsoluteIndexedY, cpu),
        LdaZp => lda(ZeroPage, cpu),
        LdaZpX => lda(ZeroPageIndexedX, cpu),
        LdaIndX => lda(IndexedIndirectX, cpu),
        LdaIndY => lda(IndirectIndexedY, cpu),

        StaAbs => sta(Absolute, cpu),
        StaAbsX => sta(AbsoluteIndexedX, cpu),
        StaAbsY => sta(AbsoluteIndexedY, cpu),
        StaZp => sta(ZeroPage, cpu),
        StaZpX => sta(ZeroPageIndexedX, cpu),
        StaIndX => sta(IndexedIndirectX, cpu),
        StaIndY => sta(IndirectIndexedY, cpu),
        _ => todo!("OpCode: 0x{:2X}", op_code as u8),
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
