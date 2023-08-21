use crate::ppu::PpuRegisters;
pub use cartridge::Cartridge;
pub use cpu::{Cpu, CpuMemory};
pub use ppu::{Ppu, PpuMemory};
use std::cell::RefCell;
use std::rc::Rc;

mod cartridge;
mod cpu;

mod ppu;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Nes {
    pub cpu: Cpu,
    pub cpu_mem: CpuMemory,
    pub ppu: Ppu,
    pub ppu_mem: PpuMemory,
    pub cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl Nes {
    pub fn insert_cartridge(&mut self, cartridge: Option<Cartridge>) -> Option<Cartridge> {
        let cartridge = cartridge.map(|c| Rc::new(RefCell::new(c)));
        let old = std::mem::replace(&mut self.cartridge, cartridge);
        self.cpu_mem.connect_cartridge(self.cartridge.clone());
        self.ppu_mem.connect_cartridge(self.cartridge.clone());
        old.map(|c| Rc::try_unwrap(c).unwrap().into_inner())
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.cpu_mem);
        self.ppu.reset(&mut self.ppu_mem);
    }

    pub fn next(&mut self) {
        self.cpu.next_cycle(&mut self.cpu_mem);
        self.ppu.next_cycle(&mut self.ppu_mem);
        self.ppu.next_cycle(&mut self.ppu_mem);
        self.ppu.next_cycle(&mut self.ppu_mem);
    }

    pub fn next_cpu_operation(&mut self) {
        self.next();

        while self.cpu.state.is_some() {
            self.next();
        }
    }
}

impl Default for Nes {
    fn default() -> Self {
        let registers = Rc::new(RefCell::new(PpuRegisters::default()));

        Self {
            cpu: Default::default(),
            cpu_mem: CpuMemory::new(registers.clone()),
            ppu: Ppu::new(registers),
            ppu_mem: Default::default(),
            cartridge: None,
        }
    }
}
