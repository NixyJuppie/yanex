macro_rules! gen_imm_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn imm() {
            let cpu = crate::Cpu::default();

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_abs_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn abs() {
            let cpu = crate::Cpu::default();

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x0010u16.to_le_bytes()[0]);
            crate::MemoryAccess::write_u8(&mut memory, 0x0002, 0x0010u16.to_le_bytes()[1]);

            crate::MemoryAccess::write_u8(&mut memory, 0x0010, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_abs_x_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn abs_x() {
            let mut cpu = crate::Cpu::default();
            cpu.registers.index_x = 0x04;

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x0010u16.to_le_bytes()[0]);
            crate::MemoryAccess::write_u8(&mut memory, 0x0002, 0x0010u16.to_le_bytes()[1]);

            crate::MemoryAccess::write_u8(&mut memory, 0x0010 + 0x04 as u16, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_abs_y_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn abs_y() {
            let mut cpu = crate::Cpu::default();
            cpu.registers.index_y = 0x04;

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x0010u16.to_le_bytes()[0]);
            crate::MemoryAccess::write_u8(&mut memory, 0x0002, 0x0010u16.to_le_bytes()[1]);

            crate::MemoryAccess::write_u8(&mut memory, 0x0010 + 0x04 as u16, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_zp_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn zp() {
            let cpu = crate::Cpu::default();

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x10);

            crate::MemoryAccess::write_u8(&mut memory, 0x0010, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_zp_x_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn zp_x() {
            let mut cpu = crate::Cpu::default();
            cpu.registers.index_x = 0x04;

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x10);

            crate::MemoryAccess::write_u8(&mut memory, 0x0014, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_zp_y_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn zp_y() {
            let mut cpu = crate::Cpu::default();
            cpu.registers.index_y = 0x04;

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x10);

            crate::MemoryAccess::write_u8(&mut memory, 0x0014, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_ind_x_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn ind_x() {
            let mut cpu = crate::Cpu::default();
            cpu.registers.index_x = 0x04;

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x10);

            crate::MemoryAccess::write_u8(&mut memory, 0x0014, 0x20);
            crate::MemoryAccess::write_u8(&mut memory, 0x0015, 0x01);
            crate::MemoryAccess::write_u8(&mut memory, 0x0120, $data);

            $assert(cpu, memory);
        }
    };
}

macro_rules! gen_ind_y_test {
    ($opcode:ident, $data:literal, $assert:expr) => {
        #[test]
        fn ind_y() {
            let mut cpu = crate::Cpu::default();
            cpu.registers.index_y = 0x04;

            let mut memory = crate::Memory::default();
            crate::MemoryAccess::write_opcode(&mut memory, 0x0000, $opcode);
            crate::MemoryAccess::write_u8(&mut memory, 0x0001, 0x10);

            crate::MemoryAccess::write_u8(&mut memory, 0x0010, 0x20);
            crate::MemoryAccess::write_u8(&mut memory, 0x0011, 0x01);
            crate::MemoryAccess::write_u8(&mut memory, 0x0124, $data);

            $assert(cpu, memory);
        }
    };
}

pub(crate) use gen_abs_test;
pub(crate) use gen_abs_x_test;
pub(crate) use gen_abs_y_test;
pub(crate) use gen_imm_test;
pub(crate) use gen_ind_x_test;
pub(crate) use gen_ind_y_test;
pub(crate) use gen_zp_test;
pub(crate) use gen_zp_x_test;
pub(crate) use gen_zp_y_test;
