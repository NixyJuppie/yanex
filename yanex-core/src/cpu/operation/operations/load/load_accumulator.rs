use crate::cpu::operation::addressing_mode::{
    AddressingMode, AddressingModeRead, AddressingModeReadDataState,
};
use crate::cpu::CpuRegisters;
use crate::CpuMemory;

#[derive(Debug, Clone)]
pub enum LoadAccumulatorState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl LoadAccumulatorState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
        match self {
            LoadAccumulatorState::Decoded(ref mode) => {
                self.try_execute(registers, memory, (*mode).into())
            }
            LoadAccumulatorState::Fetching(ref mode) => {
                self.try_execute(registers, memory, mode.clone())
            }
            LoadAccumulatorState::Executed => true,
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
                *self = LoadAccumulatorState::Fetching(read_state);
                false
            }
            Some(data) => {
                registers.accumulator = data;
                registers.status.set_negative(data);
                registers.status.set_zero(data);

                *self = LoadAccumulatorState::Executed;
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
    use crate::CpuMemory;
    use crate::Opcode::{LdaAbs, LdaAbsX, LdaAbsY, LdaImm, LdaIndX, LdaIndY, LdaZp, LdaZpX};

    fn assert() -> fn(Cpu, CpuMemory) {
        |mut cpu: Cpu, mut memory: CpuMemory| {
            cpu.next_operation(&mut memory, &mut None);
            assert_eq!(cpu.registers.accumulator, 0xFF);
            assert!(!cpu.registers.status.b1_zero);
            assert!(cpu.registers.status.b7_negative);
        }
    }

    gen_imm_test!(LdaImm, 0xFF, assert());
    gen_imm_cycles_test!(LdaImm, 2);

    gen_abs_test!(LdaAbs, 0xFF, assert());
    gen_abs_cycles_test!(LdaAbs, 4);

    gen_abs_x_test!(LdaAbsX, 0xFF, assert());
    gen_abs_x_cycles_test!(LdaAbsX, 4);
    gen_abs_x_page_crossed_cycles_test!(LdaAbsX, 5);

    gen_abs_y_test!(LdaAbsY, 0xFF, assert());
    gen_abs_y_cycles_test!(LdaAbsY, 4);
    gen_abs_y_page_crossed_cycles_test!(LdaAbsY, 5);

    gen_zp_test!(LdaZp, 0xFF, assert());
    gen_zp_cycles_test!(LdaZp, 3);

    gen_zp_x_test!(LdaZpX, 0xFF, assert());
    gen_zp_x_cycles_test!(LdaZpX, 4);

    gen_ind_x_test!(LdaIndX, 0xFF, assert());
    gen_ind_x_cycles_test!(LdaIndX, 6);

    gen_ind_y_test!(LdaIndY, 0xFF, assert());
    gen_ind_y_cycles_test!(LdaIndY, 5);
    gen_ind_y_page_crossed_cycles_test!(LdaIndY, 6);
}
