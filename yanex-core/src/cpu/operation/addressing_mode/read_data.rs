use crate::cpu::{Cpu, CpuMemory};

#[derive(Debug, Clone)]
pub enum AddressingModeReadData {
    Implied(ImpliedAddressingModeReadData),
    Immediate(ImmediateAddressingModeReadData),
    ZeroPage(ZeroPageAddressingModeReadData),
    Absolute(AbsoluteAddressingModeReadData),
    IndirectX(IndirectXAddressingModeReadData),
}

impl AddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        match self {
            AddressingModeReadData::Implied(state) => state.read(cpu, memory),
            AddressingModeReadData::Immediate(state) => state.read(cpu, memory),
            AddressingModeReadData::ZeroPage(state) => state.read(cpu, memory),
            AddressingModeReadData::Absolute(state) => state.read(cpu, memory),
            AddressingModeReadData::IndirectX(state) => state.read(cpu, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImpliedAddressingModeReadData;

impl ImpliedAddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        // Dummy read
        Some(memory.read_u8(cpu.registers.program_counter))
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImmediateAddressingModeReadData;

impl ImmediateAddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        let data = memory.read_u8(cpu.registers.program_counter);
        cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);
        Some(data)
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageAddressingModeReadData {
    #[default]
    None,
    AddressLowByte(u8),
}

impl ZeroPageAddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        match self {
            ZeroPageAddressingModeReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = ZeroPageAddressingModeReadData::AddressLowByte(low_byte);
                None
            }
            ZeroPageAddressingModeReadData::AddressLowByte(low_byte) => {
                let address = u16::from_le_bytes([*low_byte, 0x00]);
                let data = memory.read_u8(address);

                Some(data)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteAddressingModeReadData {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
}

impl AbsoluteAddressingModeReadData {
    pub fn read(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> Option<u8> {
        match self {
            AbsoluteAddressingModeReadData::None => {
                let low_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = AbsoluteAddressingModeReadData::AddressLowByte(low_byte);
                None
            }
            AbsoluteAddressingModeReadData::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(cpu.registers.program_counter);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                let address = u16::from_le_bytes([*low_byte, high_byte]);
                *self = AbsoluteAddressingModeReadData::Address(address);
                None
            }
            AbsoluteAddressingModeReadData::Address(address) => {
                let data = memory.read_u8(*address);
                Some(data)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectXAddressingModeReadData {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
    Address(u16),
}

impl IndirectXAddressingModeReadData {
    fn read(&mut self, cpu: &mut Cpu, memory: &CpuMemory) -> Option<u8> {
        match self {
            IndirectXAddressingModeReadData::None => {
                let low_byte = memory
                    .read_u8(cpu.registers.program_counter)
                    .wrapping_add(cpu.registers.index_x);
                cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(1);

                *self = IndirectXAddressingModeReadData::PointerLowByte(low_byte);
                None
            }
            IndirectXAddressingModeReadData::PointerLowByte(low_byte) => {
                memory.read_u8(cpu.registers.program_counter);

                *self = IndirectXAddressingModeReadData::DummyRead(*low_byte);
                None
            }
            IndirectXAddressingModeReadData::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let low_byte = memory.read_u8(pointer);

                *self =
                    IndirectXAddressingModeReadData::AddressLowByte(low_byte, *pointer_low_byte);
                None
            }
            IndirectXAddressingModeReadData::AddressLowByte(low_byte, pointer_low_byte) => {
                let pointer = u16::from_le_bytes([pointer_low_byte.wrapping_add(1), 0x00]);
                let high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectXAddressingModeReadData::Address(address);
                None
            }
            IndirectXAddressingModeReadData::Address(address) => Some(memory.read_u8(*address)),
        }
    }
}
