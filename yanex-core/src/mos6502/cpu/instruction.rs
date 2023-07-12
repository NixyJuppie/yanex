use crate::mos6502::cpu::addressing_mode::AddressingMode;
use crate::mos6502::cpu::op_code::OpCode;
use crate::mos6502::cpu::Cpu;

pub fn execute_instruction(op_code: OpCode, cpu: &mut Cpu) {
    use crate::mos6502::cpu::addressing_mode::AddressingMode::*;
    use crate::mos6502::cpu::op_code::OpCode::*;

    match op_code {
        // Load
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
        // Transfer
        TaxImp => tax(cpu),
        TayImp => tay(cpu),
        TxaImp => txa(cpu),
        TyaImp => tya(cpu),
        TsxImp => tsx(cpu),
        TxsImp => txs(cpu),
        // Stack
        PhaImp => pha(cpu),
        PhpImp => php(cpu),
        PlaImp => pla(cpu),
        PlpImp => plp(cpu),
        // Shift
        AslAcc => asl(Accumulator, cpu),
        AslAbs => asl(Absolute, cpu),
        AslAbsX => asl(AbsoluteIndexedX, cpu),
        AslZp => asl(ZeroPage, cpu),
        AslZpX => asl(ZeroPageIndexedX, cpu),
        LsrAcc => lsr(Accumulator, cpu),
        LsrAbs => lsr(Absolute, cpu),
        LsrAbsX => lsr(AbsoluteIndexedX, cpu),
        LsrZp => lsr(ZeroPage, cpu),
        LsrZpX => lsr(ZeroPageIndexedX, cpu),
        RolAcc => rol(Accumulator, cpu),
        RolAbs => rol(Absolute, cpu),
        RolAbsX => rol(AbsoluteIndexedX, cpu),
        RolZp => rol(ZeroPage, cpu),
        RolZpX => rol(ZeroPageIndexedX, cpu),
        RorAcc => ror(Accumulator, cpu),
        RorAbs => ror(Absolute, cpu),
        RorAbsX => ror(AbsoluteIndexedX, cpu),
        RorZp => ror(ZeroPage, cpu),
        RorZpX => ror(ZeroPageIndexedX, cpu),
        // Logic
        BitAbs => bit(Absolute, cpu),
        BitZp => bit(ZeroPage, cpu),
        AndImm => and(Immediate, cpu),
        AndAbs => and(Absolute, cpu),
        AndAbsX => and(AbsoluteIndexedX, cpu),
        AndAbsY => and(AbsoluteIndexedY, cpu),
        AndZp => and(ZeroPage, cpu),
        AndZpX => and(ZeroPageIndexedX, cpu),
        AndIndX => and(IndexedIndirectX, cpu),
        AndIndY => and(IndirectIndexedY, cpu),
        OraImm => ora(Immediate, cpu),
        OraAbs => ora(Absolute, cpu),
        OraAbsX => ora(AbsoluteIndexedX, cpu),
        OraAbsY => ora(AbsoluteIndexedY, cpu),
        OraZp => ora(ZeroPage, cpu),
        OraZpX => ora(ZeroPageIndexedX, cpu),
        OraIndX => ora(IndexedIndirectX, cpu),
        OraIndY => ora(IndirectIndexedY, cpu),
        EorImm => eor(Immediate, cpu),
        EorAbs => eor(Absolute, cpu),
        EorAbsX => eor(AbsoluteIndexedX, cpu),
        EorAbsY => eor(AbsoluteIndexedY, cpu),
        EorZp => eor(ZeroPage, cpu),
        EorZpX => eor(ZeroPageIndexedX, cpu),
        EorIndX => eor(IndexedIndirectX, cpu),
        EorIndY => eor(IndirectIndexedY, cpu),

        // TODO
        NopImp => nop(),
        JmpAbs => jmp(Absolute, cpu),
        JmpInd => jmp(Indirect, cpu),
        _ => todo!("OpCode: 0x{:2X}", op_code as u8),
    }
}

fn lda(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = addressing_mode.read_data(cpu, true);
    cpu.registers.accumulator = data;
    cpu.registers.status.z = data == 0;
    cpu.registers.status.n = data & 0b1000_0000 == 0b1000_0000;
}

fn ldx(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = addressing_mode.read_data(cpu, true);
    cpu.registers.index_x = data;
    cpu.registers.status.z = data == 0;
    cpu.registers.status.n = data & 0b1000_0000 == 0b1000_0000;
}

fn ldy(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = addressing_mode.read_data(cpu, true);
    cpu.registers.index_y = data;
    cpu.registers.status.z = data == 0;
    cpu.registers.status.n = data & 0b1000_0000 == 0b1000_0000;
}

fn sta(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = cpu.registers.accumulator;
    addressing_mode.write(data, cpu, true);
}

fn stx(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = cpu.registers.index_x;
    addressing_mode.write(data, cpu, true);
}

fn sty(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let data = cpu.registers.index_y;
    addressing_mode.write(data, cpu, true);
}

fn tax(cpu: &mut Cpu) {
    cpu.registers.index_x = cpu.registers.accumulator;
    cpu.registers.status.z = cpu.registers.accumulator == 0;
    cpu.registers.status.n = cpu.registers.accumulator & 0b1000_0000 == 0b1000_0000;
}

fn tay(cpu: &mut Cpu) {
    cpu.registers.index_y = cpu.registers.accumulator;
    cpu.registers.status.z = cpu.registers.accumulator == 0;
    cpu.registers.status.n = cpu.registers.accumulator & 0b1000_0000 == 0b1000_0000;
}

fn txa(cpu: &mut Cpu) {
    cpu.registers.accumulator = cpu.registers.index_x;
    cpu.registers.status.z = cpu.registers.index_x == 0;
    cpu.registers.status.n = cpu.registers.index_x & 0b1000_0000 == 0b1000_0000;
}

fn tya(cpu: &mut Cpu) {
    cpu.registers.accumulator = cpu.registers.index_y;
    cpu.registers.status.z = cpu.registers.index_y == 0;
    cpu.registers.status.n = cpu.registers.index_y & 0b1000_0000 == 0b1000_0000;
}

fn tsx(cpu: &mut Cpu) {
    cpu.registers.index_x = cpu.registers.stack_pointer;
    cpu.registers.status.z = cpu.registers.stack_pointer == 0;
    cpu.registers.status.n = cpu.registers.stack_pointer & 0b1000_0000 == 0b1000_0000;
}

fn txs(cpu: &mut Cpu) {
    cpu.registers.stack_pointer = cpu.registers.index_x;
}

fn pha(cpu: &mut Cpu) {
    cpu.write_stack(cpu.registers.stack_pointer, cpu.registers.accumulator);
    cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(1);
}

fn php(cpu: &mut Cpu) {
    cpu.write_stack(cpu.registers.stack_pointer, cpu.registers.status.into())
}

fn pla(cpu: &mut Cpu) {
    cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_add(1);
    cpu.registers.accumulator = cpu.read_stack(cpu.registers.stack_pointer);
}

fn plp(cpu: &mut Cpu) {
    cpu.registers.status = cpu.read_stack(cpu.registers.stack_pointer).into()
}

fn asl(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, false);
    let result = value << 1;

    addressing_mode.write(result, cpu, true);
    cpu.registers.status.z = result == 0;
    cpu.registers.status.n = result & 0b1000_0000 == 0b1000_0000;
    cpu.registers.status.c = value & 0b1000_0000 == 0b1000_0000;
}

fn lsr(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, false);
    let result = value >> 1;

    addressing_mode.write(result, cpu, true);
    cpu.registers.status.z = result == 0;
    cpu.registers.status.n = false;
    cpu.registers.status.c = value & 0b0000_0001 == 0b0000_0001;
}

fn rol(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, false);
    let mut result = value << 1;
    if cpu.registers.status.c {
        result |= 0b0000_0001
    }

    addressing_mode.write(result, cpu, true);
    cpu.registers.status.z = result == 0;
    cpu.registers.status.n = result & 0b1000_0000 == 0b1000_0000;
    cpu.registers.status.c = value & 0b1000_0000 == 0b1000_0000;
}

fn ror(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, false);
    let mut result = value >> 1;
    if cpu.registers.status.c {
        result |= 0b1000_0000
    }

    addressing_mode.write(result, cpu, true);
    cpu.registers.status.z = result == 0;
    cpu.registers.status.n = cpu.registers.status.c;
    cpu.registers.status.c = value & 0b0000_0001 == 0b0000_0001;
}

fn bit(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, true);

    cpu.registers.status.z = value & cpu.registers.accumulator == 0;
    cpu.registers.status.n = value & 0b1000_0000 == 0b1000_0000;
    cpu.registers.status.v = value & 0b0100_0000 == 0b0100_0000;
}

fn and(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, false);
    cpu.registers.accumulator &= value;

    cpu.registers.status.z = cpu.registers.accumulator == 0;
    cpu.registers.status.n = cpu.registers.accumulator & 0b1000_0000 == 0b1000_0000;
}

fn ora(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, false);
    cpu.registers.accumulator |= value;

    cpu.registers.status.z = cpu.registers.accumulator == 0;
    cpu.registers.status.n = cpu.registers.accumulator & 0b1000_0000 == 0b1000_0000;
}

fn eor(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    let value = addressing_mode.read_data(cpu, false);
    cpu.registers.accumulator ^= value;

    cpu.registers.status.z = cpu.registers.accumulator == 0;
    cpu.registers.status.n = cpu.registers.accumulator & 0b1000_0000 == 0b1000_0000;
}

// TODO

fn jmp(addressing_mode: AddressingMode, cpu: &mut Cpu) {
    cpu.registers.program_counter = addressing_mode.read_address(cpu, true);
}

fn nop() {}
