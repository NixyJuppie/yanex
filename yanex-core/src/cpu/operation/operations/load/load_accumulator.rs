use crate::cpu::operation::addressing_mode::{
    AddressingMode, AddressingModeRead, AddressingModeReadDataState,
};
use crate::cpu::CpuRegisters;
use crate::Memory;

#[derive(Debug, Clone)]
pub enum LoadAccumulatorState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl LoadAccumulatorState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut Memory) -> bool {
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
        memory: &Memory,
        mut read_state: AddressingModeReadDataState,
    ) -> bool {
        match read_state.advance(registers, memory) {
            None => {
                *self = LoadAccumulatorState::Fetching(read_state);
                false
            }
            Some(data) => {
                registers.accumulator = data;
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
    use crate::Memory;
    use crate::Opcode::{LdaAbs, LdaAbsX, LdaAbsY, LdaImm, LdaIndX, LdaIndY, LdaZp, LdaZpX};

    fn assert() -> fn(Cpu, Memory) {
        |mut cpu: Cpu, mut memory: Memory| {
            cpu.execute_operation(&mut memory, &mut None);
            assert_eq!(cpu.registers.accumulator, 0x42);
        }
    }

    gen_imm_test!(LdaImm, 0x42, assert());
    gen_imm_cycles_test!(LdaImm, 2);

    gen_abs_test!(LdaAbs, 0x42, assert());
    gen_abs_cycles_test!(LdaAbs, 4);

    gen_abs_x_test!(LdaAbsX, 0x42, assert());
    gen_abs_x_cycles_test!(LdaAbsX, 4);
    gen_abs_x_page_crossed_cycles_test!(LdaAbsX, 5);

    gen_abs_y_test!(LdaAbsY, 0x42, assert());
    gen_abs_y_cycles_test!(LdaAbsY, 4);
    gen_abs_y_page_crossed_cycles_test!(LdaAbsY, 5);

    gen_zp_test!(LdaZp, 0x42, assert());
    gen_zp_cycles_test!(LdaZp, 3);

    gen_zp_x_test!(LdaZpX, 0x42, assert());
    gen_zp_x_cycles_test!(LdaZpX, 4);

    gen_ind_x_test!(LdaIndX, 0x42, assert());
    gen_ind_x_cycles_test!(LdaIndX, 6);

    gen_ind_y_test!(LdaIndY, 0x42, assert());
    gen_ind_y_cycles_test!(LdaIndY, 5);
    gen_ind_y_page_crossed_cycles_test!(LdaIndY, 6);
}
