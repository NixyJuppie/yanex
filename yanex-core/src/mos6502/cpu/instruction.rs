use crate::mos6502::cpu::addressing_mode::AddressingMode;
use crate::mos6502::cpu::op_code::OpCode;
use crate::mos6502::cpu::Cpu;

pub fn execute_instruction(op_code: OpCode, cpu: &mut Cpu) {
    use crate::mos6502::cpu::addressing_mode::AddressingMode::*;
    use crate::mos6502::cpu::op_code::OpCode::*;

    match op_code {
        LdaImm => lda(Immediate, cpu),
        LdaAbs => lda(Absolute, cpu),
        LdaAbsX => lda(AbsoluteIndexedX, cpu),
        LdaAbsY => lda(AbsoluteIndexedY, cpu),
        LdaZp => lda(ZeroPage, cpu),
        LdaZpX => lda(ZeroPageIndexedX, cpu),
        LdaIndX => lda(IndexedIndirectX, cpu),
        LdaIndY => lda(IndirectIndexedY, cpu),

        LdxImm => ldx(Immediate, cpu),
        LdxAbs => ldx(Absolute, cpu),
        LdxAbsY => ldx(AbsoluteIndexedY, cpu),
        LdxZp => ldx(ZeroPage, cpu),
        LdxZpY => ldx(ZeroPageIndexedY, cpu),

        LdyImm => ldy(Immediate, cpu),
        LdyAbs => ldy(Absolute, cpu),
        LdyAbsX => ldy(AbsoluteIndexedX, cpu),
        LdyZp => ldy(ZeroPage, cpu),
        LdyZpX => ldy(ZeroPageIndexedX, cpu),

        StaAbs => sta(Absolute, cpu),
        StaAbsX => sta(AbsoluteIndexedX, cpu),
        StaAbsY => sta(AbsoluteIndexedY, cpu),
        StaZp => sta(ZeroPage, cpu),
        StaZpX => sta(ZeroPageIndexedX, cpu),
        StaIndX => sta(IndexedIndirectX, cpu),
        StaIndY => sta(IndirectIndexedY, cpu),

        StxAbs => stx(Absolute, cpu),
        StxZp => stx(ZeroPage, cpu),
        StxZpY => stx(ZeroPageIndexedY, cpu),

        StyAbs => sty(Absolute, cpu),
        StyZp => sty(ZeroPage, cpu),
        StyZpX => sty(ZeroPageIndexedX, cpu),

        NopImp => {}

        JmpAbs => jmp(Absolute, cpu),
        JmpInd => jmp(Indirect, cpu),
        _ => todo!("OpCode: 0x{:2X}", op_code as u8),
    }
}

fn lda(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = addressing_mode.read_data(cpu);
    cpu.registers.accumulator = data;
    cpu.registers.status.z = data == 0;
    cpu.registers.status.n = data & 0b10000000 == 0b10000000;
}

fn ldx(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = addressing_mode.read_data(cpu);
    cpu.registers.index_x = data;
    cpu.registers.status.z = data == 0;
    cpu.registers.status.n = data & 0b10000000 == 0b10000000;
}

fn ldy(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = addressing_mode.read_data(cpu);
    cpu.registers.index_y = data;
    cpu.registers.status.z = data == 0;
    cpu.registers.status.n = data & 0b10000000 == 0b10000000;
}

fn sta(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = cpu.registers.accumulator;
    addressing_mode.write(data, cpu);
}

fn stx(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = cpu.registers.index_x;
    addressing_mode.write(data, cpu);
}

fn sty(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = cpu.registers.index_y;
    addressing_mode.write(data, cpu);
}

fn jmp(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    cpu.registers.program_counter = addressing_mode.read_address(cpu);
}
