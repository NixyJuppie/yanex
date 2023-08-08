use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeReadAddress {
    Relative(RelativeAddressingModeReadAddress),
    ZeroPage(ZeroPageAddressingModeReadAddress),
    Absolute(AbsoluteAddressingModeReadAddress),
    Indirect(IndirectAddressingModeReadAddress),
    IndirectX(IndirectXAddressingModeReadAddress),
    IndirectY(IndirectYAddressingModeReadAddress),
}

impl AddressingModeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u16> {
        match self {
            AddressingModeReadAddress::Relative(state) => state.read(cpu, memory),
            AddressingModeReadAddress::ZeroPage(state) => state.read(cpu, memory),
            AddressingModeReadAddress::Absolute(state) => state.read(cpu, memory),
            AddressingModeReadAddress::Indirect(state) => state.read(cpu, memory),
            AddressingModeReadAddress::IndirectX(state) => state.read(cpu, memory),
            AddressingModeReadAddress::IndirectY(state) => state.read(cpu, memory),
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

#[derive(Debug, Default, Clone)]
pub enum IndirectXAddressingModeReadAddress {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
}

impl IndirectXAddressingModeReadAddress {
    fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            IndirectXAddressingModeReadAddress::None => {
                let low_byte = memory
                    .read_u8(cpu.registers.program_counter)
                    .wrapping_add(cpu.registers.index_x);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectXAddressingModeReadAddress::PointerLowByte(low_byte);
                None
            }
            IndirectXAddressingModeReadAddress::PointerLowByte(low_byte) => {
                memory.read_u8(cpu.registers.program_counter);

                *self = IndirectXAddressingModeReadAddress::DummyRead(*low_byte);
                None
            }
            IndirectXAddressingModeReadAddress::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self =
                    IndirectXAddressingModeReadAddress::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectXAddressingModeReadAddress::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);
                Some(address)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectYAddressingModeReadAddress {
    #[default]
    None,
    PointerLowByte(u8),
    AddressLowByte(u8, u8),
    PageCrossed(u16),
}

impl IndirectYAddressingModeReadAddress {
    fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            IndirectYAddressingModeReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectYAddressingModeReadAddress::PointerLowByte(low_byte);
                None
            }
            IndirectYAddressingModeReadAddress::PointerLowByte(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self =
                    IndirectYAddressingModeReadAddress::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectYAddressingModeReadAddress::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let low_byte_y = low_byte.wrapping_add(cpu.registers.index_y);

                if low_byte_y < *low_byte {
                    let address = u16::from_le_bytes([low_byte_y, high_byte.wrapping_add(1)]);
                    *self = IndirectYAddressingModeReadAddress::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte_y, high_byte]);
                    Some(address)
                }
            }
            IndirectYAddressingModeReadAddress::PageCrossed(address) => Some(*address),
        }
    }
}
