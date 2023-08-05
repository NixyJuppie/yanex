use crate::cpu::addressing_mode::AddressingModeReadData;
use crate::cpu::addressing_mode::AddressingModeReadDataState;
use crate::cpu::AddressingMode;
use crate::cpu::CpuRegisters;
use crate::CpuMemory;

#[derive(Debug, Clone)]
pub enum LoadIndexYState {
    Decoded(AddressingMode),
    Fetching(AddressingModeReadDataState),
    Executed,
}

impl LoadIndexYState {
    pub fn advance(&mut self, registers: &mut CpuRegisters, memory: &mut CpuMemory) -> bool {
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
        memory: &CpuMemory,
        mut read_state: AddressingModeReadDataState,
    ) -> bool {
        match read_state.advance(registers, memory) {
            None => {
                *self = LoadIndexYState::Fetching(read_state);
                false
            }
            Some(data) => {
                registers.index_y = data;
                registers.status.set_negative_value(data);
                registers.status.set_zero_value(data);

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
    use crate::CpuMemory;

    fn assert() -> fn(Cpu, CpuMemory) {
        |mut cpu: Cpu, mut memory: CpuMemory| {
            cpu.next_operation(&mut memory);
            assert_eq!(cpu.registers.index_y, 0xFF);
            assert!(!cpu.registers.status.zero());
            assert!(cpu.registers.status.negative());
        }
    }

    gen_imm_test!(0xA0, 0xFF, assert());
    gen_imm_cycles_test!(0xA0, 2);

    gen_abs_test!(0xAC, 0xFF, assert());
    gen_abs_cycles_test!(0xAC, 4);

    gen_abs_x_test!(0xBC, 0xFF, assert());
    gen_abs_x_cycles_test!(0xBC, 4);
    gen_abs_x_page_crossed_cycles_test!(0xBC, 5);

    gen_zp_test!(0xA4, 0xFF, assert());
    gen_zp_cycles_test!(0xA4, 3);

    gen_zp_x_test!(0xB4, 0xFF, assert());
    gen_zp_x_cycles_test!(0xB4, 4);
}
