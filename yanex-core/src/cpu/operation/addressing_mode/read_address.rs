use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeReadAddress {
    ZeroPage(ZeroPageAddressingModeReadAddress),
    Absolute(AbsoluteAddressingModeReadAddress),
    Indirect(IndirectAddressingModeReadAddress),
}

impl AddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            AddressingModeReadAddress::ZeroPage(state) => state.read(cpu, memory),
            AddressingModeReadAddress::Absolute(state) => state.read(cpu, memory),
            AddressingModeReadAddress::Indirect(state) => state.read(cpu, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ZeroPageAddressingModeReadAddress;

impl ZeroPageAddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        cpu.internal_registers.address_low_byte = memory.read_u8(cpu.registers.program_counter);
        cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
        Some(u16::from_le_bytes([
            cpu.internal_registers.address_low_byte,
            0x00,
        ]))
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteAddressingModeReadAddress {
    #[default]
    None,
    AddressLowByte,
}

impl AbsoluteAddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            AbsoluteAddressingModeReadAddress::None => {
                cpu.internal_registers.address_low_byte =
                    memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
                *self = AbsoluteAddressingModeReadAddress::AddressLowByte;
                None
            }
            AbsoluteAddressingModeReadAddress::AddressLowByte => {
                cpu.internal_registers.address_high_byte =
                    memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
                Some(u16::from_le_bytes([
                    cpu.internal_registers.address_low_byte,
                    cpu.internal_registers.address_high_byte,
                ]))
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectAddressingModeReadAddress {
    #[default]
    None,
    PointerLowByte,
    Pointer,
    AddressLowByte,
}

impl IndirectAddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            IndirectAddressingModeReadAddress::None => {
                cpu.internal_registers.pointer_low_byte =
                    memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectAddressingModeReadAddress::PointerLowByte;
                None
            }
            IndirectAddressingModeReadAddress::PointerLowByte => {
                cpu.internal_registers.pointer_high_byte =
                    memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectAddressingModeReadAddress::Pointer;
                None
            }
            IndirectAddressingModeReadAddress::Pointer => {
                cpu.internal_registers.address_low_byte =
                    memory.read_u8(cpu.internal_registers.pointer());

                *self = IndirectAddressingModeReadAddress::AddressLowByte;
                None
            }
            IndirectAddressingModeReadAddress::AddressLowByte => {
                // Hardware bug: only low byte is incremented
                // On low byte overflow, high byte is not changed
                // e.g. 0x42FF => (0x42FF, 0x4200)
                cpu.internal_registers.pointer_low_byte =
                    cpu.internal_registers.pointer_low_byte.wrapping_add(1);
                cpu.internal_registers.address_high_byte =
                    memory.read_u8(cpu.internal_registers.pointer());

                *self = IndirectAddressingModeReadAddress::AddressLowByte;
                Some(cpu.internal_registers.address())
            }
        }
    }
}
