use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum ReadData {
    Implied(ImpliedReadData),
    Immediate(ImmediateReadData),
    ZeroPage(ZeroPageReadData),
    ZeroPageX(ZeroPageXReadData),
    ZeroPageY(ZeroPageYReadData),
    Absolute(AbsoluteReadData),
    AbsoluteX(AbsoluteXReadData),
    AbsoluteY(AbsoluteYReadData),
    IndirectX(IndirectXReadData),
    IndirectY(IndirectYReadData),
}

impl ReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            ReadData::Implied(state) => state.read(cpu, memory),
            ReadData::Immediate(state) => state.read(cpu, memory),
            ReadData::ZeroPage(state) => state.read(cpu, memory),
            ReadData::ZeroPageX(state) => state.read(cpu, memory),
            ReadData::ZeroPageY(state) => state.read(cpu, memory),
            ReadData::Absolute(state) => state.read(cpu, memory),
            ReadData::AbsoluteX(state) => state.read(cpu, memory),
            ReadData::AbsoluteY(state) => state.read(cpu, memory),
            ReadData::IndirectX(state) => state.read(cpu, memory),
            ReadData::IndirectY(state) => state.read(cpu, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImpliedReadData;

impl ImpliedReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        // Dummy read
        Some(memory.read_u8(cpu.registers.program_counter))
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImmediateReadData;

impl ImmediateReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        let data = memory.read_u8(cpu.registers.program_counter);
        cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
        Some(data)
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageReadData {
    #[default]
    None,
    AddressLowByte(u8),
}

impl ZeroPageReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            ZeroPageReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageReadData::AddressLowByte(low_byte);
                None
            }
            ZeroPageReadData::AddressLowByte(low_byte) => {
                let address = u16::from_le_bytes([*low_byte, 0x00]);
                let data = memory.read_u8(address);

                Some(data)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageXReadData {
    #[default]
    None,
    DummyRead(u8),
    AddressLowByte(u8),
}

impl ZeroPageXReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            ZeroPageXReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageXReadData::DummyRead(low_byte);
                None
            }
            ZeroPageXReadData::DummyRead(low_byte) => {
                *self = ZeroPageXReadData::AddressLowByte(*low_byte);
                None
            }
            ZeroPageXReadData::AddressLowByte(low_byte) => {
                let low_byte = low_byte.wrapping_add(cpu.registers.index_x);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                let data = memory.read_u8(address);

                Some(data)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageYReadData {
    #[default]
    None,
    DummyRead(u8),
    AddressLowByte(u8),
}

impl ZeroPageYReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            ZeroPageYReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageYReadData::DummyRead(low_byte);
                None
            }
            ZeroPageYReadData::DummyRead(low_byte) => {
                *self = ZeroPageYReadData::AddressLowByte(*low_byte);
                None
            }
            ZeroPageYReadData::AddressLowByte(low_byte) => {
                let low_byte = low_byte.wrapping_add(cpu.registers.index_y);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                let data = memory.read_u8(address);

                Some(data)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteReadData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
}

impl AbsoluteReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            AbsoluteReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteReadData::AddressLowByte(low_byte);
                None
            }
            AbsoluteReadData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteReadData::Address(address);
                None
            }
            AbsoluteReadData::Address(address) => {
                let data = memory.read_u8(*address);
                Some(data)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteXReadData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
    PageCrossed(u16),
}

impl AbsoluteXReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            AbsoluteXReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteXReadData::AddressLowByte(low_byte);
                None
            }
            AbsoluteXReadData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteXReadData::Address(address);
                None
            }
            AbsoluteXReadData::Address(address) => {
                let address_bytes = address.to_le_bytes();
                let low_byte = address_bytes[0].wrapping_add(cpu.registers.index_x);

                if low_byte < address_bytes[0] {
                    let high_byte = address_bytes[1].wrapping_add(1);
                    let address = u16::from_le_bytes([low_byte, high_byte]);
                    *self = AbsoluteXReadData::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte, address_bytes[1]]);
                    Some(memory.read_u8(address))
                }
            }
            AbsoluteXReadData::PageCrossed(address) => Some(memory.read_u8(*address)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteYReadData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
    PageCrossed(u16),
}

impl AbsoluteYReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            AbsoluteYReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteYReadData::AddressLowByte(low_byte);
                None
            }
            AbsoluteYReadData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteYReadData::Address(address);
                None
            }
            AbsoluteYReadData::Address(address) => {
                let address_bytes = address.to_le_bytes();
                let low_byte = address_bytes[0].wrapping_add(cpu.registers.index_y);

                if low_byte < address_bytes[0] {
                    let high_byte = address_bytes[1].wrapping_add(1);
                    let address = u16::from_le_bytes([low_byte, high_byte]);
                    *self = AbsoluteYReadData::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte, address_bytes[1]]);
                    Some(memory.read_u8(address))
                }
            }
            AbsoluteYReadData::PageCrossed(address) => Some(memory.read_u8(*address)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectXReadData {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
    Address(u16),
}

impl IndirectXReadData {
    fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            IndirectXReadData::None => {
                let low_byte = memory
                    .read_u8(cpu.registers.program_counter)
                    .wrapping_add(cpu.registers.index_x);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectXReadData::PointerLowByte(low_byte);
                None
            }
            IndirectXReadData::PointerLowByte(low_byte) => {
                memory.read_u8(cpu.registers.program_counter);

                *self = IndirectXReadData::DummyRead(*low_byte);
                None
            }
            IndirectXReadData::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self = IndirectXReadData::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectXReadData::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectXReadData::Address(address);
                None
            }
            IndirectXReadData::Address(address) => Some(memory.read_u8(*address)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectYReadData {
    #[default]
    None,
    PointerLowByte(u8),
    AddressLowByte(u8, u8),
    Address(u16),
    PageCrossed(u16),
}

impl IndirectYReadData {
    fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            IndirectYReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectYReadData::PointerLowByte(low_byte);
                None
            }
            IndirectYReadData::PointerLowByte(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self = IndirectYReadData::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectYReadData::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectYReadData::Address(address);
                None
            }
            IndirectYReadData::Address(address) => {
                let address_bytes = address.to_le_bytes();
                let low_byte = address_bytes[0].wrapping_add(cpu.registers.index_y);

                if low_byte < address_bytes[0] {
                    let high_byte = address_bytes[1].wrapping_add(1);
                    let address = u16::from_le_bytes([low_byte, high_byte]);
                    *self = IndirectYReadData::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte, address_bytes[1]]);
                    Some(memory.read_u8(address))
                }
            }
            IndirectYReadData::PageCrossed(address) => Some(memory.read_u8(*address)),
        }
    }
}
