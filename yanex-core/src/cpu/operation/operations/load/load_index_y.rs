use crate::cpu::operation::addressing_mode::{
    AddressingMode, AddressingModeRead, AddressingModeReadDataState,
};
use crate::cpu::CpuRegisters;
use crate::Memory;

#[derive(Debug, Clone)]
pub enum LoadIndexYState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl LoadIndexYState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) -> bool {
        match self {
            LoadIndexYState::Decoded(ref mode) => {
                self.try_execute(registers, memory, (*mode).into())
            }
            LoadIndexYState::Fetching(ref mode) => {
                self.try_execute(registers, memory, mode.clone())
            }
            LoadIndexYState::Executed => true,
        }
    }

    fn try_execute(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &Memory,
        mut read_state: AddressingModeReadDataState,
    ) -> bool {
        match read_state.advance(registers, memory) {
            None => {
                *self = LoadIndexYState::Fetching(read_state);
                false
            }
            Some(data) => {
                registers.index_y = data;
                registers.status.set_negative(data);
                registers.status.set_zero(data);

                *self = LoadIndexYState::Executed;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_utils::cycles_macros::*;
    use crate::tests_utils::opcode_macros::*;
    use crate::Cpu;
    use crate::Memory;
    use crate::Opcode::{LdyAbs, LdyAbsX, LdyImm, LdyZp, LdyZpX};

    fn assert() -> fn(Cpu, Memory) {
        |mut cpu: Cpu, mut memory: Memory| {
            cpu.next_operation(&mut memory, &mut None);
            assert_eq!(cpu.registers.index_y, 0xFF);
            assert!(!cpu.registers.status.b1_zero);
            assert!(cpu.registers.status.b7_negative);
        }
    }

    gen_imm_test!(LdyImm, 0xFF, assert());
    gen_imm_cycles_test!(LdyImm, 2);

    gen_abs_test!(LdyAbs, 0xFF, assert());
    gen_abs_cycles_test!(LdyAbs, 4);

    gen_abs_x_test!(LdyAbsX, 0xFF, assert());
    gen_abs_x_cycles_test!(LdyAbsX, 4);
    gen_abs_x_page_crossed_cycles_test!(LdyAbsX, 5);

    gen_zp_test!(LdyZp, 0xFF, assert());
    gen_zp_cycles_test!(LdyZp, 3);

    gen_zp_x_test!(LdyZpX, 0xFF, assert());
    gen_zp_x_cycles_test!(LdyZpX, 4);
}
