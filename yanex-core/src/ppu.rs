mod memory;

pub use memory::PpuMemory;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Ppu {
    _registers: Rc<RefCell<PpuRegisters>>,
}

impl Ppu {
    pub fn new(registers: Rc<RefCell<PpuRegisters>>) -> Self {
        Self {
            _registers: registers,
        }
    }

    pub fn reset(&mut self, _memory: &mut PpuMemory) {
        // todo!()
    }

    pub fn next_cycle(&mut self, _memory: &mut PpuMemory) {
        // todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PpuRegisters {
    data: [u8; 8],
}

impl PpuRegisters {
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize & 0x07]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize & 0x07] = data;
    }
}
