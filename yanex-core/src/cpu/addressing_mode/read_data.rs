use crate::{CpuMemory, CpuRegisters, MemoryAccess};

pub trait AddressingModeReadData {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8>;
}

#[derive(Debug, Clone)]
pub enum AddressingModeReadDataState {
    Implied(ImpliedReadDataState),
    Immediate(ImmediateReadDataState),
    ZeroPage(ZeroPageReadDataState),
    ZeroPageX(ZeroPageXReadDataState),
    ZeroPageY(ZeroPageYReadDataState),
    Absolute(AbsoluteReadDataState),
    AbsoluteX(AbsoluteXReadDataState),
    AbsoluteY(AbsoluteYReadDataState),
    IndirectX(IndirectXReadDataState),
    IndirectY(IndirectYReadDataState),
}

impl AddressingModeReadData for AddressingModeReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        use AddressingModeReadDataState::*;

        match self {
            Implied(state) => state.advance(registers, memory),
            Immediate(state) => state.advance(registers, memory),
            ZeroPage(state) => state.advance(registers, memory),
            ZeroPageX(state) => state.advance(registers, memory),
            ZeroPageY(state) => state.advance(registers, memory),
            Absolute(state) => state.advance(registers, memory),
            AbsoluteX(state) => state.advance(registers, memory),
            AbsoluteY(state) => state.advance(registers, memory),
            IndirectX(state) => state.advance(registers, memory),
            IndirectY(state) => state.advance(registers, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ImpliedReadDataState {
    #[default]
    None,
    DummyRead(u8),
}

impl AddressingModeReadData for ImpliedReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            ImpliedReadDataState::None => {
                // Dummy read without incrementing PC
                let data = memory.read_u8(registers.program_counter);

                *self = ImpliedReadDataState::DummyRead(data);
                Some(data)
            }
            ImpliedReadDataState::DummyRead(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ImmediateReadDataState {
    #[default]
    None,
    Data(u8),
}

impl AddressingModeReadData for ImmediateReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            ImmediateReadDataState::None => {
                let data = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ImmediateReadDataState::Data(data);
                Some(data)
            }
            ImmediateReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Data(u8),
}

impl AddressingModeReadData for ZeroPageReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            ZeroPageReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ZeroPageReadDataState::AddressLowByte(low_byte);
                None
            }
            ZeroPageReadDataState::AddressLowByte(low_byte) => {
                const HIGH_BYTE: u8 = 0x00;
                let address = u16::from_le_bytes([*low_byte, HIGH_BYTE]);
                let data = memory.read_u8(address);

                *self = ZeroPageReadDataState::Data(data);
                Some(data)
            }
            ZeroPageReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageXReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    DummyRead(u8),
    Data(u8),
}

impl AddressingModeReadData for ZeroPageXReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            ZeroPageXReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ZeroPageXReadDataState::AddressLowByte(low_byte);
                None
            }
            ZeroPageXReadDataState::AddressLowByte(low_byte) => {
                memory.read_u8(u16::from_le_bytes([*low_byte, 0x00]));

                *self = ZeroPageXReadDataState::DummyRead(*low_byte);
                None
            }
            ZeroPageXReadDataState::DummyRead(low_byte) => {
                let low_byte = low_byte.wrapping_add(registers.index_x);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                let data = memory.read_u8(address);

                *self = ZeroPageXReadDataState::Data(data);
                Some(data)
            }
            ZeroPageXReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ZeroPageYReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    DummyRead(u8),
    Data(u8),
}

impl AddressingModeReadData for ZeroPageYReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            ZeroPageYReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = ZeroPageYReadDataState::AddressLowByte(low_byte);
                None
            }
            ZeroPageYReadDataState::AddressLowByte(low_byte) => {
                memory.read_u8(u16::from_le_bytes([*low_byte, 0x00]));

                *self = ZeroPageYReadDataState::DummyRead(*low_byte);
                None
            }
            ZeroPageYReadDataState::DummyRead(low_byte) => {
                let low_byte = low_byte.wrapping_add(registers.index_y);
                let address = u16::from_le_bytes([low_byte, 0x00]);
                let data = memory.read_u8(address);

                *self = ZeroPageYReadDataState::Data(data);
                Some(data)
            }
            ZeroPageYReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
    Data(u8),
}

impl AddressingModeReadData for AbsoluteReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            AbsoluteReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteReadDataState::AddressLowByte(low_byte);
                None
            }
            AbsoluteReadDataState::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteReadDataState::Address(u16::from_le_bytes([*low_byte, high_byte]));
                None
            }
            AbsoluteReadDataState::Address(address) => {
                let data = memory.read_u8(*address);

                *self = AbsoluteReadDataState::Data(data);
                Some(data)
            }
            AbsoluteReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteXReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
    PageCrossed(u16),
    Data(u8),
}

impl AddressingModeReadData for AbsoluteXReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            AbsoluteXReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteXReadDataState::AddressLowByte(low_byte);
                None
            }
            AbsoluteXReadDataState::AddressLowByte(original_low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                let low_byte = original_low_byte.wrapping_add(registers.index_x);
                let address = u16::from_le_bytes([low_byte, high_byte]);

                if low_byte >= *original_low_byte {
                    *self = AbsoluteXReadDataState::Address(address);
                } else {
                    // Overflow, page crossed
                    *self = AbsoluteXReadDataState::PageCrossed(address);
                }

                None
            }
            AbsoluteXReadDataState::PageCrossed(address) => {
                let bytes = address.to_le_bytes();
                let address = u16::from_le_bytes([bytes[0], bytes[1].wrapping_add(1)]);
                *self = AbsoluteXReadDataState::Address(address);
                None
            }
            AbsoluteXReadDataState::Address(address) => {
                let data = memory.read_u8(*address);

                *self = AbsoluteXReadDataState::Data(data);
                Some(data)
            }
            AbsoluteXReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteYReadDataState {
    #[default]
    None,
    AddressLowByte(u8),
    PageCrossed(u16),
    Address(u16),
    Data(u8),
}

impl AddressingModeReadData for AbsoluteYReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            AbsoluteYReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteYReadDataState::AddressLowByte(low_byte);
                None
            }
            AbsoluteYReadDataState::AddressLowByte(original_low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                let low_byte = original_low_byte.wrapping_add(registers.index_y);
                let address = u16::from_le_bytes([low_byte, high_byte]);

                if low_byte >= *original_low_byte {
                    *self = AbsoluteYReadDataState::Address(address);
                } else {
                    // Overflow, page crossed
                    *self = AbsoluteYReadDataState::PageCrossed(address);
                }

                None
            }
            AbsoluteYReadDataState::PageCrossed(address) => {
                let bytes = address.to_le_bytes();
                let address = u16::from_le_bytes([bytes[0], bytes[1].wrapping_add(1)]);

                *self = AbsoluteYReadDataState::Address(address);
                None
            }
            AbsoluteYReadDataState::Address(address) => {
                let data = memory.read_u8(*address);

                *self = AbsoluteYReadDataState::Data(data);
                Some(data)
            }
            AbsoluteYReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectXReadDataState {
    #[default]
    None,
    PointerLowByte(u8),
    DummyRead(u8),
    AddressLowByte(u8, u8),
    Address(u16),
    Data(u8),
}

impl AddressingModeReadData for IndirectXReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            IndirectXReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = IndirectXReadDataState::PointerLowByte(low_byte);
                None
            }
            IndirectXReadDataState::PointerLowByte(low_byte) => {
                let pointer = u16::from_le_bytes([*low_byte, 0x00]);
                memory.read_u8(pointer);

                *self = IndirectXReadDataState::DummyRead(*low_byte + registers.index_x);
                None
            }
            IndirectXReadDataState::DummyRead(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_low_byte = memory.read_u8(pointer);
                let pointer_low_byte = pointer_low_byte.wrapping_add(1);

                *self = IndirectXReadDataState::AddressLowByte(pointer_low_byte, address_low_byte);
                None
            }
            IndirectXReadDataState::AddressLowByte(pointer_low_byte, address_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_high_byte = memory.read_u8(pointer);
                let address = u16::from_le_bytes([*address_low_byte, address_high_byte]);

                *self = IndirectXReadDataState::Address(address);
                None
            }
            IndirectXReadDataState::Address(address) => {
                let data = memory.read_u8(*address);

                Some(data)
            }
            IndirectXReadDataState::Data(data) => Some(*data),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectYReadDataState {
    #[default]
    None,
    PointerLowByte(u8),
    AddressLowByte(u8, u8),
    PageCrossed(u16),
    Address(u16),
    Data(u8),
}

impl AddressingModeReadData for IndirectYReadDataState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u8> {
        match self {
            IndirectYReadDataState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = IndirectYReadDataState::PointerLowByte(low_byte);
                None
            }
            IndirectYReadDataState::PointerLowByte(pointer_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_low_byte = memory.read_u8(pointer);
                let pointer_low_byte = pointer_low_byte.wrapping_add(1);

                *self = IndirectYReadDataState::AddressLowByte(pointer_low_byte, address_low_byte);
                None
            }
            IndirectYReadDataState::AddressLowByte(pointer_low_byte, original_address_low_byte) => {
                let pointer = u16::from_le_bytes([*pointer_low_byte, 0x00]);
                let address_high_byte = memory.read_u8(pointer);

                let address_low_byte = original_address_low_byte.wrapping_add(registers.index_y);
                let address = u16::from_le_bytes([address_low_byte, address_high_byte]);

                if address_low_byte >= *original_address_low_byte {
                    *self = IndirectYReadDataState::Address(address);
                } else {
                    // Overflow, page crossed
                    *self = IndirectYReadDataState::PageCrossed(address)
                }

                None
            }
            IndirectYReadDataState::PageCrossed(address) => {
                let bytes = address.to_le_bytes();
                let address = u16::from_le_bytes([bytes[0], bytes[1].wrapping_add(1)]);

                *self = IndirectYReadDataState::Address(address);
                None
            }
            IndirectYReadDataState::Address(address) => {
                let data = memory.read_u8(*address);

                *self = IndirectYReadDataState::Data(data);
                Some(data)
            }
            IndirectYReadDataState::Data(data) => Some(*data),
        }
    }
}
