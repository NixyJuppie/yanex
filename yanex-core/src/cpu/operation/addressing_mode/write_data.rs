use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeWriteData {
    ZeroPage(u8, ZeroPageAddressingModeWriteData),
    Absolute(u8, AbsoluteAddressingModeWriteData),
    IndirectX(u8, IndirectXAddressingModeWriteData),
    IndirectY(u8, IndirectYAddressingModeWriteData),
}

impl AddressingModeWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<()> {
        match self {
            AddressingModeWriteData::ZeroPage(data, state) => state.write(cpu, memory, *data),
            AddressingModeWriteData::Absolute(data, state) => state.write(cpu, memory, *data),
            AddressingModeWriteData::IndirectX(data, state) => state.write(cpu, memory, *data),
            AddressingModeWriteData::IndirectY(data, state) => state.write(cpu, memory, *data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageAddressingModeWriteData {
    #[default]
    None,
    AddressLowByte(u8),
}

impl ZeroPageAddressingModeWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            ZeroPageAddressingModeWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageAddressingModeWriteData::AddressLowByte(low_byte);
                None
            }
            ZeroPageAddressingModeWriteData::AddressLowByte(low_byte) => {
                let address = u16::from_le_bytes([*low_byte, 0x00]);
                memory.write_u8(address, data);

                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteAddressingModeWriteData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
}

impl AbsoluteAddressingModeWriteData {
    pub fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            AbsoluteAddressingModeWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteAddressingModeWriteData::AddressLowByte(low_byte);
                None
            }
            AbsoluteAddressingModeWriteData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteAddressingModeWriteData::Address(address);
                None
            }
            AbsoluteAddressingModeWriteData::Address(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectXAddressingModeWriteData {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
    Address(u16),
}

impl IndirectXAddressingModeWriteData {
    fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            IndirectXAddressingModeWriteData::None => {
                let low_byte = memory
                    .read_u8(cpu.registers.program_counter)
                    .wrapping_add(cpu.registers.index_x);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectXAddressingModeWriteData::PointerLowByte(low_byte);
                None
            }
            IndirectXAddressingModeWriteData::PointerLowByte(low_byte) => {
                memory.read_u8(cpu.registers.program_counter);

                *self = IndirectXAddressingModeWriteData::DummyRead(*low_byte);
                None
            }
            IndirectXAddressingModeWriteData::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self =
                    IndirectXAddressingModeWriteData::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectXAddressingModeWriteData::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectXAddressingModeWriteData::Address(address);
                None
            }
            IndirectXAddressingModeWriteData::Address(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectYAddressingModeWriteData {
    #[default]
    None,
    PointerLowByte(u8),
    AddressLowByte(u8, u8),
    DummyRead(u8, u8),
    Address(u16),
    PageCrossed(u16),
}

impl IndirectYAddressingModeWriteData {
    fn write(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory, data: u8) -> Option<()> {
        match self {
            IndirectYAddressingModeWriteData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectYAddressingModeWriteData::PointerLowByte(low_byte);
                None
            }
            IndirectYAddressingModeWriteData::PointerLowByte(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self =
                    IndirectYAddressingModeWriteData::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectYAddressingModeWriteData::AddressLowByte(low_byte, pointer_low_byte) => {
                *self = IndirectYAddressingModeWriteData::DummyRead(*low_byte, *pointer_low_byte);
                None
            }
            IndirectYAddressingModeWriteData::DummyRead(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectYAddressingModeWriteData::Address(address);
                None
            }
            IndirectYAddressingModeWriteData::Address(address) => {
                let address_bytes = address.to_le_bytes();
                let low_byte = address_bytes[0].wrapping_add(cpu.registers.index_y);

                if low_byte < address_bytes[0] {
                    let high_byte = address_bytes[1].wrapping_add(1);
                    let address = u16::from_le_bytes([low_byte, high_byte]);
                    *self = IndirectYAddressingModeWriteData::PageCrossed(address);
                    None
                } else {
                    let address = u16::from_le_bytes([low_byte, address_bytes[1]]);
                    memory.write_u8(address, data);
                    Some(())
                }
            }
            IndirectYAddressingModeWriteData::PageCrossed(address) => {
                memory.write_u8(*address, data);
                Some(())
            }
        }
    }
}
