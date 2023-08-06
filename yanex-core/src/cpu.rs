use bitfield_struct::bitfield;
pub use memory::CpuMemory;
pub use operation::{AddressingMode, Opcode, Operation};

mod memory;
mod operation;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: CpuRegisters,
    pub internal_registers: CpuInternalRegisters,
    pub state: Option<Operation>,
    pub cycle: usize,
}

impl Cpu {
    pub fn reset(&mut self, memory: &mut CpuMemory) {
        self.registers = CpuRegisters {
            stack_pointer: 0xFD,
            ..Default::default()
        };
        self.internal_registers = CpuInternalRegisters::default();
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

#[derive(Debug, Default)]
pub struct CpuInternalRegisters {
    pub pointer_low_byte: u8,
    pub pointer_high_byte: u8,
    pub address_low_byte: u8,
    pub address_high_byte: u8,
}

impl CpuInternalRegisters {
    pub fn pointer(&self) -> u16 {
        u16::from_le_bytes([self.pointer_low_byte, self.pointer_high_byte])
    }
    pub fn address(&self) -> u16 {
        u16::from_le_bytes([self.address_low_byte, self.address_high_byte])
    }
}
