use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum WriteData {
    ZeroPage(u8, ZeroPageWriteData),
    ZeroPageX(u8, ZeroPageXWriteData),
    ZeroPageY(u8, ZeroPageYWriteData),
    Absolute(u8, AbsoluteWriteData),
    AbsoluteX(u8, AbsoluteXWriteData),
    AbsoluteY(u8, AbsoluteYWriteData),
    IndirectX(u8, IndirectXWriteData),
    IndirectY(u8, IndirectYWriteData),
}

impl WriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            WriteData::ZeroPage(data, state) => state.write(cpu, memory, *data),
            WriteData::ZeroPageX(data, state) => state.write(cpu, memory, *data),
            WriteData::ZeroPageY(data, state) => state.write(cpu, memory, *data),
            WriteData::Absolute(data, state) => state.write(cpu, memory, *data),
            WriteData::AbsoluteX(data, state) => state.write(cpu, memory, *data),
            WriteData::AbsoluteY(data, state) => state.write(cpu, memory, *data),
            WriteData::IndirectX(data, state) => state.write(cpu, memory, *data),
            WriteData::IndirectY(data, state) => state.write(cpu, memory, *data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageWriteData {
    #[default]
    None,
    AddressLowByte(u8),
}

impl ZeroPageWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            ZeroPageWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageWriteData::AddressLowByte(low_byte);
                None
            }
            ZeroPageWriteData::AddressLowByte(low_byte) => {
                let address = u16::from_le_bytes([*low_byte, 0x00]);
                memory.write_u8(address, data);

                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageXWriteData {
    #[default]
    None,
    DummyRead(u8),
    AddressLowByte(u8),
}

impl ZeroPageXWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            ZeroPageXWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageXWriteData::DummyRead(low_byte);
                None
            }
            ZeroPageXWriteData::DummyRead(low_byte) => {
                *self = ZeroPageXWriteData::AddressLowByte(*low_byte);
                None
            }
            ZeroPageXWriteData::AddressLowByte(low_byte) => {
                let low_byte = low_byte.wrapping_add(cpu.registers.index_x);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                memory.write_u8(address, data);

                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageYWriteData {
    #[default]
    None,
    DummyRead(u8),
    AddressLowByte(u8),
}

impl ZeroPageYWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            ZeroPageYWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageYWriteData::DummyRead(low_byte);
                None
            }
            ZeroPageYWriteData::DummyRead(low_byte) => {
                *self = ZeroPageYWriteData::AddressLowByte(*low_byte);
                None
            }
            ZeroPageYWriteData::AddressLowByte(low_byte) => {
                let low_byte = low_byte.wrapping_add(cpu.registers.index_y);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                memory.write_u8(address, data);

                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteWriteData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
}

impl AbsoluteWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            AbsoluteWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteWriteData::AddressLowByte(low_byte);
                None
            }
            AbsoluteWriteData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteWriteData::Address(address);
                None
            }
            AbsoluteWriteData::Address(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteXWriteData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
    DummyRead(u16),
}

impl AbsoluteXWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            AbsoluteXWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteXWriteData::AddressLowByte(low_byte);
                None
            }
            AbsoluteXWriteData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteXWriteData::Address(address);
                None
            }
            AbsoluteXWriteData::Address(address) => {
                let address_bytes = address.to_le_bytes();
                let low_byte = address_bytes[0].wrapping_add(cpu.registers.index_x);
                let high_byte = if low_byte < address_bytes[0] {
                    address_bytes[1].wrapping_add(1)
                } else {
                    address_bytes[1]
                };
                let address = u16::from_le_bytes([low_byte, high_byte]);

                *self = AbsoluteXWriteData::DummyRead(address);
                None
            }
            AbsoluteXWriteData::DummyRead(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteYWriteData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
    DummyRead(u16),
}

impl AbsoluteYWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            AbsoluteYWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteYWriteData::AddressLowByte(low_byte);
                None
            }
            AbsoluteYWriteData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteYWriteData::Address(address);
                None
            }
            AbsoluteYWriteData::Address(address) => {
                let address_bytes = address.to_le_bytes();
                let low_byte = address_bytes[0].wrapping_add(cpu.registers.index_y);
                let high_byte = if low_byte < address_bytes[0] {
                    address_bytes[1].wrapping_add(1)
                } else {
                    address_bytes[1]
                };
                let address = u16::from_le_bytes([low_byte, high_byte]);

                *self = AbsoluteYWriteData::DummyRead(address);
                None
            }
            AbsoluteYWriteData::DummyRead(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectXWriteData {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
    Address(u16),
}

impl IndirectXWriteData {
    fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            IndirectXWriteData::None => {
                let low_byte = memory
                    .read_u8(cpu.registers.program_counter)
                    .wrapping_add(cpu.registers.index_x);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectXWriteData::PointerLowByte(low_byte);
                None
            }
            IndirectXWriteData::PointerLowByte(low_byte) => {
                memory.read_u8(cpu.registers.program_counter);

                *self = IndirectXWriteData::DummyRead(*low_byte);
                None
            }
            IndirectXWriteData::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self = IndirectXWriteData::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectXWriteData::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectXWriteData::Address(address);
                None
            }
            IndirectXWriteData::Address(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectYWriteData {
    #[default]
    None,
    PointerLowByte(u8),
    AddressLowByte(u8, u8),
    DummyRead(u8, u8),
    Address(u16),
    PageCrossed(u16),
}

impl IndirectYWriteData {
    fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            IndirectYWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectYWriteData::PointerLowByte(low_byte);
                None
            }
            IndirectYWriteData::PointerLowByte(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self = IndirectYWriteData::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectYWriteData::AddressLowByte(low_byte, pointer_low_byte) => {
                *self = IndirectYWriteData::DummyRead(*low_byte, *pointer_low_byte);
                None
            }
            IndirectYWriteData::DummyRead(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectYWriteData::Address(address);
                None
            }
            IndirectYWriteData::Address(address) => {
                let address_bytes = address.to_le_bytes();
                let low_byte = address_bytes[0].wrapping_add(cpu.registers.index_y);

                if low_byte < address_bytes[0] {
                    let high_byte = address_bytes[1].wrapping_add(1);
                    let address = u16::from_le_bytes([low_byte, high_byte]);
                    *self = IndirectYWriteData::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte, address_bytes[1]]);
                    memory.write_u8(address, data);
                    Some(())
                }
            }
            IndirectYWriteData::PageCrossed(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}
