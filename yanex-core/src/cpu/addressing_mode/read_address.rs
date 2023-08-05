use crate::{CpuMemory, CpuRegisters, MemoryAccess};

pub trait AddressingModeReadAddress {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u16>;
}

#[derive(Debug, Clone)]
pub enum AddressingModeReadAddressState {
    Absolute(AbsoluteReadAddressState),
    Indirect(IndirectReadAddressState),
}

impl AddressingModeReadAddress for AddressingModeReadAddressState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u16> {
        use AddressingModeReadAddressState::*;

        match self {
            Absolute(state) => state.advance(registers, memory),
            Indirect(state) => state.advance(registers, memory),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum AbsoluteReadAddressState {
    #[default]
    None,
    AddressLowByte(u8),
    Address(u16),
}

impl AddressingModeReadAddress for AbsoluteReadAddressState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u16> {
        match self {
            AbsoluteReadAddressState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = AbsoluteReadAddressState::AddressLowByte(low_byte);
                None
            }
            AbsoluteReadAddressState::AddressLowByte(low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;
                let address = u16::from_le_bytes([*low_byte, high_byte]);

                *self = AbsoluteReadAddressState::Address(address);
                Some(address)
            }
            AbsoluteReadAddressState::Address(address) => Some(*address),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum IndirectReadAddressState {
    #[default]
    None,
    PointerLowByte(u8),
    Pointer(u16),
    AddressLowByte(u8, u16),
    Address(u16),
}

impl AddressingModeReadAddress for IndirectReadAddressState {
    fn advance(&mut self, registers: &mut CpuRegisters, memory: &CpuMemory) -> Option<u16> {
        match self {
            IndirectReadAddressState::None => {
                let low_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;

                *self = IndirectReadAddressState::PointerLowByte(low_byte);
                None
            }
            IndirectReadAddressState::PointerLowByte(low_byte) => {
                let high_byte = memory.read_u8(registers.program_counter);
                registers.program_counter += 1;
                let pointer = u16::from_le_bytes([*low_byte, high_byte]);

                *self = IndirectReadAddressState::Pointer(pointer);
                None
            }
            IndirectReadAddressState::Pointer(pointer) => {
                let address_low_byte = memory.read_u8(*pointer);

                // Bug in MOS 6502 CPU, pointer high byte is not incremented on page crossing
                // Expected: 0x42FF => (*0x42FF, *0x4300)
                // Actual: 0x42FF => (*0x42FF, *0x4200)
                let pointer_bytes = pointer.to_le_bytes();
                let pointer_bytes = [pointer_bytes[0].wrapping_add(1), pointer_bytes[1]];
                let pointer = u16::from_le_bytes(pointer_bytes);

                *self = IndirectReadAddressState::AddressLowByte(address_low_byte, pointer);
                None
            }
            IndirectReadAddressState::AddressLowByte(address_low_byte, pointer) => {
                let address_high_byte = memory.read_u8(*pointer);
                let address = u16::from_le_bytes([*address_low_byte, address_high_byte]);

                *self = IndirectReadAddressState::Address(address);
                Some(address)
            }
            IndirectReadAddressState::Address(address) => Some(*address),
        }
    }
}
