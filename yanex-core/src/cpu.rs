use bitfield_struct::bitfield;
pub use memory::CpuMemory;
pub use operation::{AddressingMode, Opcode, Operation};

mod memory;
mod operation;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: CpuRegisters,
    pub state: Option<Operation>,
    pub cycle: usize,
}

impl Cpu {
    pub fn reset(&mut self, memory: &mut CpuMemory) {
        self.registers = CpuRegisters {
            stack_pointer: 0xFD,
            ..Default::default()
        };
        self.state = None;
        self.cycle = 0;

        // Temporarily fill RAM with init code
        memory.write_u8(0x0000, Opcode::SeiImp as u8);
        memory.write_u8(0x0001, Opcode::CldImp as u8);
        memory.write_u8(0x0002, Opcode::JmpAbs as u8);
        memory.write_u16(0x0003, memory.read_u16(0xFFFC));

        self.next_operation(memory);
        self.next_operation(memory);
        self.next_operation(memory);

        // Clear
        memory.write_u8(0x0000, 0x00);
        memory.write_u8(0x0000, 0x00);
        memory.write_u8(0x0000, 0x00);
        memory.write_u16(0x0000, 0x0000);
    }

    pub fn next_cycle(&mut self, memory: &mut CpuMemory) {
        self.state = match self.state.as_ref() {
            None => {
                let opcode: Opcode = memory.read_u8(self.registers.program_counter).into();
                self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
                Some(opcode.into())
            }
            Some(operation) => {
                let mut operation = operation.clone();
                if operation.execute(self, memory) {
                    None
                } else {
                    Some(operation)
                }
            }
        };

        self.cycle += 1;
    }

    pub fn next_operation(&mut self, memory: &mut CpuMemory) {
        self.next_cycle(memory);

        while self.state.is_some() {
            self.next_cycle(memory)
        }
    }

    pub fn stack_push(&mut self, memory: &mut CpuMemory, data: u8) {
        memory.write_u8(0x0100 + self.registers.stack_pointer as u16, data);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    pub fn stack_pull(&mut self, memory: &mut CpuMemory) -> u8 {
        let data = memory.read_u8(0x0100 + self.registers.stack_pointer as u16);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        data
    }
}

#[derive(Debug, Default)]
pub struct CpuRegisters {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status: CpuStatus,
}

#[bitfield(u8)]
pub struct CpuStatus {
    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub _break: bool,
    pub unused: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl CpuStatus {
    pub fn set_zero_and_negative(&mut self, value: u8) {
        self.set_zero(value == 0);
        self.set_negative(value & 0b1000_0000 != 0);
    }
}
