use crate::cpu::operation::addressing_mode::{
    AddressingMode, AddressingModeRead, AddressingModeReadDataState,
};
use crate::cpu::CpuRegisters;
use crate::CpuMemory;

#[derive(Debug, Clone)]
pub enum LoadIndexXState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl LoadIndexXState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
        match self {
            LoadIndexXState::Decoded(ref mode) => {
                self.try_execute(registers, memory, (*mode).into())
            }
            LoadIndexXState::Fetching(ref mode) => {
                self.try_execute(registers, memory, mode.clone())
            }
            LoadIndexXState::Executed => true,
        }
    }

    fn try_execute(
        &mut self,
        registers: &mut CpuRegisters,
        memory: &CpuMemory,
        mut read_state: AddressingModeReadDataState,
    ) -> bool {
        match read_state.advance(registers, memory) {
            None => {
                *self = LoadIndexXState::Fetching(read_state);
                false
            }
            Some(data) => {
                registers.index_x = data;
                registers.status.set_negative(data);
                registers.status.set_zero(data);

                *self = LoadIndexXState::Executed;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_utils::cycles_macros::*;
    use crate::tests_utils::opcode_macros::*;
    use crate::CpuMemory;
    use crate::Opcode::{LdxAbs, LdxAbsY, LdxImm, LdxZp, LdxZpY};
    use crate::{Cpu, CpuStatus};

    fn assert() -> fn(Cpu, CpuMemory) {
        |mut cpu: Cpu, mut memory: CpuMemory| {
            cpu.next_operation(&mut memory);
            assert_eq!(cpu.registers.index_x, 0xFF);
            assert!(!cpu.registers.status.contains(CpuStatus::B1_ZERO));
            assert!(cpu.registers.status.contains(CpuStatus::B7_NEGATIVE));
        }
    }

    gen_imm_test!(LdxImm, 0xFF, assert());
    gen_imm_cycles_test!(LdxImm, 2);

    gen_abs_test!(LdxAbs, 0xFF, assert());
    gen_abs_cycles_test!(LdxAbs, 4);

    gen_abs_y_test!(LdxAbsY, 0xFF, assert());
    gen_abs_y_cycles_test!(LdxAbsY, 4);
    gen_abs_y_page_crossed_cycles_test!(LdxAbsY, 5);

    gen_zp_test!(LdxZp, 0xFF, assert());
    gen_zp_cycles_test!(LdxZp, 3);

    gen_zp_y_test!(LdxZpY, 0xFF, assert());
    gen_zp_y_cycles_test!(LdxZpY, 4);
}
