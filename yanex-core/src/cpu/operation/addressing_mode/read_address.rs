use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum ReadAddress {
    Relative(RelativeReadAddress),
    ZeroPage(ZeroPageReadAddress),
    ZeroPageX(ZeroPageXReadAddress),
    ZeroPageY(ZeroPageYReadAddress),
    Absolute(AbsoluteReadAddress),
    AbsoluteX(AbsoluteXReadAddress),
    AbsoluteY(AbsoluteYReadAddress),
    Indirect(IndirectReadAddress),
    IndirectX(IndirectXReadAddress),
    IndirectY(IndirectYReadAddress),
}

impl ReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            ReadAddress::Relative(state) => state.read(cpu, memory),
            ReadAddress::ZeroPage(state) => state.read(cpu, memory),
            ReadAddress::ZeroPageX(state) => state.read(cpu, memory),
            ReadAddress::ZeroPageY(state) => state.read(cpu, memory),
            ReadAddress::Absolute(state) => state.read(cpu, memory),
            ReadAddress::AbsoluteX(state) => state.read(cpu, memory),
            ReadAddress::AbsoluteY(state) => state.read(cpu, memory),
            ReadAddress::Indirect(state) => state.read(cpu, memory),
            ReadAddress::IndirectX(state) => state.read(cpu, memory),
            ReadAddress::IndirectY(state) => state.read(cpu, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum RelativeReadAddress {
    #[default]
    None,
    PageCrossed(u16),
}

impl RelativeReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            RelativeReadAddress::None => {
                let offset = memory.read_u8(cpu.registers.program_counter) as u16;
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                if offset & 0b_1000_0000 > 0 {
                    todo!("negative offset");
                }

                let address = cpu.registers.program_counter.wrapping_add(offset);
                if address.to_le_bytes()[1] == cpu.registers.program_counter.to_le_bytes()[1] {
                    Some(address)
                } else {
                    *self = RelativeReadAddress::PageCrossed(address);
                    None
                }
            }
            RelativeReadAddress::PageCrossed(address) => Some(*address),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ZeroPageReadAddress;

impl ZeroPageReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        let address_low_byte = memory.read_u8(cpu.registers.program_counter);
        cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
        Some(u16::from_le_bytes([address_low_byte, 0x00]))
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageXReadAddress {
    #[default]
    None,
    DummyRead(u8),
}

impl ZeroPageXReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            ZeroPageXReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageXReadAddress::DummyRead(low_byte);
                None
            }
            ZeroPageXReadAddress::DummyRead(low_byte) => {
                let low_byte = low_byte.wrapping_add(cpu.registers.index_x);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                Some(address)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageYReadAddress {
    #[default]
    None,
    DummyRead(u8),
}

impl ZeroPageYReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            ZeroPageYReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageYReadAddress::DummyRead(low_byte);
                None
            }
            ZeroPageYReadAddress::DummyRead(low_byte) => {
                let low_byte = low_byte.wrapping_add(cpu.registers.index_x);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                Some(address)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteReadAddress {
    #[default]
    None,
    AddressLowByte(u8),
}

impl AbsoluteReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            AbsoluteReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteReadAddress::AddressLowByte(low_byte);
                None
            }
            AbsoluteReadAddress::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                Some(u16::from_le_bytes([*low_byte, high_byte]))
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteXReadAddress {
    #[default]
    None,
    AddressLowByte(u8),
    PageCrossed(u16),
}

impl AbsoluteXReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            AbsoluteXReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteXReadAddress::AddressLowByte(low_byte);
                None
            }
            AbsoluteXReadAddress::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
                let low_byte_x = low_byte.wrapping_add(cpu.registers.index_x);

                if low_byte_x < *low_byte {
                    let high_byte = high_byte.wrapping_add(1);
                    let address = u16::from_le_bytes([low_byte_x, high_byte]);
                    *self = AbsoluteXReadAddress::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte_x, high_byte]);
                    Some(address)
                }
            }
            AbsoluteXReadAddress::PageCrossed(address) => Some(*address),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteYReadAddress {
    #[default]
    None,
    AddressLowByte(u8),
    PageCrossed(u16),
}

impl AbsoluteYReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            AbsoluteYReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteYReadAddress::AddressLowByte(low_byte);
                None
            }
            AbsoluteYReadAddress::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
                let low_byte_y = low_byte.wrapping_add(cpu.registers.index_y);

                if low_byte_y < *low_byte {
                    let high_byte = high_byte.wrapping_add(1);
                    let address = u16::from_le_bytes([low_byte_y, high_byte]);
                    *self = AbsoluteYReadAddress::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte_y, high_byte]);
                    Some(address)
                }
            }
            AbsoluteYReadAddress::PageCrossed(address) => Some(*address),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectReadAddress {
    #[default]
    None,
    PointerLowByte(u8),
    Pointer(u16),
    AddressLowByte(u8, u16),
}

impl IndirectReadAddress {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            IndirectReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectReadAddress::PointerLowByte(low_byte);
                None
            }
            IndirectReadAddress::PointerLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
                let pointer = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectReadAddress::Pointer(pointer);
                None
            }
            IndirectReadAddress::Pointer(pointer) => {
                let low_byte = memory.read_u8(*pointer);

                *self = IndirectReadAddress::AddressLowByte(low_byte, *pointer);
                None
            }
            IndirectReadAddress::AddressLowByte(low_byte, pointer) => {
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
pub enum IndirectXReadAddress {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
}

impl IndirectXReadAddress {
    fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            IndirectXReadAddress::None => {
                let low_byte = memory
                    .read_u8(cpu.registers.program_counter)
                    .wrapping_add(cpu.registers.index_x);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectXReadAddress::PointerLowByte(low_byte);
                None
            }
            IndirectXReadAddress::PointerLowByte(low_byte) => {
                memory.read_u8(cpu.registers.program_counter);

                *self = IndirectXReadAddress::DummyRead(*low_byte);
                None
            }
            IndirectXReadAddress::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self = IndirectXReadAddress::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectXReadAddress::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);
                Some(address)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectYReadAddress {
    #[default]
    None,
    PointerLowByte(u8),
    AddressLowByte(u8, u8),
    PageCrossed(u16),
}

impl IndirectYReadAddress {
    fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u16> {
        match self {
            IndirectYReadAddress::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectYReadAddress::PointerLowByte(low_byte);
                None
            }
            IndirectYReadAddress::PointerLowByte(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self = IndirectYReadAddress::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectYReadAddress::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let low_byte_y = low_byte.wrapping_add(cpu.registers.index_y);

                if low_byte_y < *low_byte {
                    let address = u16::from_le_bytes([low_byte_y, high_byte.wrapping_add(1)]);
                    *self = IndirectYReadAddress::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte_y, high_byte]);
                    Some(address)
                }
            }
            IndirectYReadAddress::PageCrossed(address) => Some(*address),
        }
    }
}
