use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeReadAddress {
    Relative(RelativeAddressingModeReadAddress),
    ZeroPage(ZeroPageAddressingModeReadAddress),
    Absolute(AbsoluteAddressingModeReadAddress),
    Indirect(IndirectAddressingModeReadAddress),
}

impl AddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            AddressingModeReadAddress::Relative(state) => state.read(cpu, memory),
            AddressingModeReadAddress::ZeroPage(state) => state.read(cpu, memory),
            AddressingModeReadAddress::Absolute(state) => state.read(cpu, memory),
            AddressingModeReadAddress::Indirect(state) => state.read(cpu, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum RelativeAddressingModeReadAddress {
    #[default]
    None,
    PageCrossed(u16),
}

impl RelativeAddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            RelativeAddressingModeReadAddress::None => {
                let offset = memory.read_u8(cpu.registers.program_counter) as u16;
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                if offset & 0b_1000_0000 > 0 {
                    todo!("negative offset");
                }

                let address = cpu.registers.program_counter.wrapping_add(offset);
                if address.to_le_bytes()[1] == cpu.registers.program_counter.to_le_bytes()[1] {
                    Some(address)
                } else {
                    *self = RelativeAddressingModeReadAddress::PageCrossed(address);
                    None
                }
            }
            RelativeAddressingModeReadAddress::PageCrossed(address) => Some(*address),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ZeroPageAddressingModeReadAddress;

impl ZeroPageAddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        let address_low_byte = memory.read_u8(cpu.registers.program_counter);
        cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
        Some(u16::from_le_bytes([address_low_byte, 0x00]))
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteAddressingModeReadAddress {
    #[default]
    None,
    AddressLowByte(u8),
}

impl AbsoluteAddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            AbsoluteAddressingModeReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteAddressingModeReadAddress::AddressLowByte(low_byte);
                None
            }
            AbsoluteAddressingModeReadAddress::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                Some(u16::from_le_bytes([*low_byte, high_byte]))
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectAddressingModeReadAddress {
    #[default]
    None,
    PointerLowByte(u8),
    Pointer(u16),
    AddressLowByte(u8, u16),
}

impl IndirectAddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            IndirectAddressingModeReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectAddressingModeReadAddress::PointerLowByte(low_byte);
                None
            }
            IndirectAddressingModeReadAddress::PointerLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
                let pointer = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectAddressingModeReadAddress::Pointer(pointer);
                None
            }
            IndirectAddressingModeReadAddress::Pointer(pointer) => {
                let low_byte = memory.read_u8(*pointer);

                *self = IndirectAddressingModeReadAddress::AddressLowByte(low_byte, *pointer);
                None
            }
            IndirectAddressingModeReadAddress::AddressLowByte(low_byte, pointer) => {
                // Hardware bug: only low byte is incremented
                // On low byte overflow, high byte is not changed
                // e.g. 0x42FF => (0x42FF, 0x4200)
                let mut pointer_bytes = pointer.to_le_bytes();
                pointer_bytes[0] = pointer_bytes[0].wrapping_add(1);
                let high_byte = memory.read_u8(u16::from_le_bytes(pointer_bytes));

                Some(u16::from_le_bytes([*low_byte, high_byte]))
            }
        }
    }
}
