use crate::cpu::addressing_mode::AddressingModeReadData;
use crate::cpu::addressing_mode::AddressingModeReadDataState;
use crate::cpu::AddressingMode;
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
                registers.status.set_negative_value(data);
                registers.status.set_zero_value(data);

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
    use crate::Cpu;
    use crate::CpuMemory;

    fn assert() -> fn(Cpu, CpuMemory) {
        |mut cpu: Cpu, mut memory: CpuMemory| {
            cpu.next_operation(&mut memory);
            assert_eq!(cpu.registers.index_x, 0xFF);
            assert!(!cpu.registers.status.zero());
            assert!(cpu.registers.status.negative());
        }
    }

    gen_imm_test!(0xA2, 0xFF, assert());
    gen_imm_cycles_test!(0xA2, 2);

    gen_abs_test!(0xAE, 0xFF, assert());
    gen_abs_cycles_test!(0xAE, 4);

    gen_abs_y_test!(0xBE, 0xFF, assert());
    gen_abs_y_cycles_test!(0xBE, 4);
    gen_abs_y_page_crossed_cycles_test!(0xBE, 5);

    gen_zp_test!(0xA6, 0xFF, assert());
    gen_zp_cycles_test!(0xA6, 3);

    gen_zp_y_test!(0xB6, 0xFF, assert());
    gen_zp_y_cycles_test!(0xB6, 4);
}
