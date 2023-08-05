use crate::cpu::addressing_mode::{AddressingModeReadAddress, AddressingModeReadAddressState};
use crate::cpu::AddressingMode;
use crate::{CpuMemory, CpuRegisters};

#[derive(Debug, Clone)]
pub enum JumpState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadAddressState),
    Executed,
}

impl JumpState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
        match self {
            JumpState::Decoded(ref mode) => self.try_execute(registers, memory, (*mode).into()),
            JumpState::Fetching(ref mode) => self.try_execute(registers, memory, mode.clone()),
            JumpState::Executed => true,
        }
    }

    fn try_execute(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &CpuMemory,
        mut read_state: AddressingModeReadAddressState,
    ) -> bool {
        match read_state.advance(registers, memory) {
            None => {
                *self = JumpState::Fetching(read_state);
                false
            }
            Some(address) => {
                registers.program_counter = address;

                *self = JumpState::Executed;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_utils::cycles_macros::*;
    use crate::CpuMemory;
    use crate::{Cpu, MemoryAccess};

    #[test]
    fn abs() {
        let mut cpu = Cpu::default();

        let mut memory = CpuMemory::default();
        memory.write_u8(0x0000, 0x4C);
        memory.write_u8(0x0001, 0x0010u16.to_le_bytes()[0]);
        memory.write_u8(0x0002, 0x0010u16.to_le_bytes()[1]);

        cpu.next_operation(&mut memory);
        assert_eq!(cpu.registers.program_counter, 0x0010);
    }

    gen_abs_cycles_test!(0x4C, 3);

    #[test]
    fn ind() {
        let mut cpu = Cpu::default();

        let mut memory = CpuMemory::default();
        memory.write_u8(0x0000, 0x6C);
        memory.write_u8(0x0001, 0x0010u16.to_le_bytes()[0]);
        memory.write_u8(0x0002, 0x0010u16.to_le_bytes()[1]);

        memory.write_u8(0x0010, 0xAAFFu16.to_le_bytes()[0]);
        memory.write_u8(0x0011, 0xAAFFu16.to_le_bytes()[1]);

        cpu.next_operation(&mut memory);
        assert_eq!(cpu.registers.program_counter, 0xAAFF);
    }

    gen_ind_cycles_test!(0x6C, 5);
}
