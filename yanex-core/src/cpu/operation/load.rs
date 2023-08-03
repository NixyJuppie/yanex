use crate::cpu::addressing_mode::AddressingModeRead;
use crate::cpu::addressing_mode::AddressingModeReadDataState;
use crate::cpu::AddressingMode;
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
                registers.status.set_negative_value(data);
                registers.status.set_zero_value(data);

                *self = LoadAccumulatorState::Executed;
                true
            }
        }
    }
}

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
    mod load_accumulator {
        use crate::tests_utils::cycles_macros::*;
        use crate::tests_utils::opcode_macros::*;
        use crate::Cpu;
        use crate::CpuMemory;

        fn assert() -> fn(Cpu, CpuMemory) {
            |mut cpu: Cpu, mut memory: CpuMemory| {
                cpu.next_operation(&mut memory);
                assert_eq!(cpu.registers.accumulator, 0xFF);
                assert!(!cpu.registers.status.zero());
                assert!(cpu.registers.status.negative());
            }
        }

        gen_imm_test!(0xA9, 0xFF, assert());
        gen_imm_cycles_test!(0xA9, 2);

        gen_abs_test!(0xAD, 0xFF, assert());
        gen_abs_cycles_test!(0xAD, 4);

        gen_abs_x_test!(0xBD, 0xFF, assert());
        gen_abs_x_cycles_test!(0xBD, 4);
        gen_abs_x_page_crossed_cycles_test!(0xBD, 5);

        gen_abs_y_test!(0xB9, 0xFF, assert());
        gen_abs_y_cycles_test!(0xB9, 4);
        gen_abs_y_page_crossed_cycles_test!(0xB9, 5);

        gen_zp_test!(0xA5, 0xFF, assert());
        gen_zp_cycles_test!(0xA5, 3);

        gen_zp_x_test!(0xB5, 0xFF, assert());
        gen_zp_x_cycles_test!(0xB5, 4);

        gen_ind_x_test!(0xA1, 0xFF, assert());
        gen_ind_x_cycles_test!(0xA1, 6);

        gen_ind_y_test!(0xB1, 0xFF, assert());
        gen_ind_y_cycles_test!(0xB1, 5);
        gen_ind_y_page_crossed_cycles_test!(0xB1, 6);
    }

    mod load_index_x {
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

    mod load_index_y {
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
}
