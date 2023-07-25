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

macro_rules! gen_imm_tests {
    ($opcode:ident, $data:literal, $cycles:expr, $assert:expr) => {
        #[test]
        fn imm() {
            let cpu = crate::Cpu::default();
            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_u8(&mut memory, 0x0000, $opcode as u8);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, $data);

            $assert(cpu, memory);
        }

        gen_cycles_test!(imm_cycles, $opcode, $cycles);
    };
}

macro_rules! gen_abs_tests {
    ($opcode:ident, $data:literal, $cycles:expr, $assert:expr) => {
        #[test]
        fn abs() {
            let cpu = crate::Cpu::default();
            let mut memory = crate::memory::Memory::default();
            crate::MemoryAccess::write_u8(&mut memory, 0x0000, $opcode as u8);

            const ADDRESS: u16 = 0x0010;
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, ADDRESS.to_le_bytes()[0]);
            crate::MemoryAccess::write_u8(&mut memory, 0x0002, ADDRESS.to_le_bytes()[1]);
            crate::MemoryAccess::write_u8(&mut memory, ADDRESS, $data);

            $assert(cpu, memory);
        }

        gen_cycles_test!(abs_cycles, $opcode, $cycles);
    };
}

pub(crate) use gen_abs_tests;
pub(crate) use gen_cycles_test;
pub(crate) use gen_imm_tests;
