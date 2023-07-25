macro_rules! gen_cycles_test {
    ($name:ident, $opcode:ident, $cycles:expr) => {
        #[test]
        fn $name() {
            let mut cpu = crate::Cpu::default();

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_u8(&mut memory, 0x0000, $opcode as u8);

            cpu.execute_operation(&mut memory, &mut None);
            assert_eq!(cpu.cycle, $cycles);
        }
    };
}

macro_rules! gen_page_crossed_cycles_test {
    ($name:ident, $opcode:ident, $cycles:expr) => {
        #[test]
        fn $name() {
            let mut cpu = crate::Cpu::default();
            cpu.registers.index_x = 0x01;
            cpu.registers.index_y = 0x01;

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_u8(&mut memory, 0x0000, $opcode as u8);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0xFF);

            cpu.execute_operation(&mut memory, &mut None);
            assert_eq!(cpu.cycle, $cycles);
        }
    };
}

macro_rules! gen_imm_cycles_test {
    ($opcode:ident, $cycles:expr) => {
        gen_cycles_test!(imm_cycles, $opcode, $cycles);
    };
}

macro_rules! gen_abs_cycles_test {
    ($opcode:ident, $cycles:expr) => {
        gen_cycles_test!(abs_cycles, $opcode, $cycles);
    };
}

macro_rules! gen_abs_x_cycles_test {
    ($opcode:ident, $cycles:expr) => {
        gen_cycles_test!(abs_x_cycles, $opcode, $cycles);
    };
}

macro_rules! gen_abs_x_page_crossed_cycles_test {
    ($opcode:ident, $cycles:expr) => {
        gen_page_crossed_cycles_test!(abs_x_page_crossed_cycles, $opcode, $cycles);
    };
}

macro_rules! gen_abs_y_cycles_test {
    ($opcode:ident, $cycles:expr) => {
        gen_cycles_test!(abs_y_cycles, $opcode, $cycles);
    };
}

macro_rules! gen_abs_y_page_crossed_cycles_test {
    ($opcode:ident, $cycles:expr) => {
        gen_page_crossed_cycles_test!(abs_y_page_crossed_cycles, $opcode, $cycles);
    };
}

pub(crate) use gen_abs_cycles_test;
pub(crate) use gen_abs_x_cycles_test;
pub(crate) use gen_abs_x_page_crossed_cycles_test;
pub(crate) use gen_abs_y_cycles_test;
pub(crate) use gen_abs_y_page_crossed_cycles_test;
pub(crate) use gen_imm_cycles_test;

pub(crate) use gen_cycles_test;
pub(crate) use gen_page_crossed_cycles_test;
