use crate::cartridge::Cartridge;
use crate::ppu::PpuRegisters;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CpuMemory {
    ram: [u8; 0x800],
    ppu_registers: Rc<RefCell<PpuRegisters>>,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl CpuMemory {
    pub fn new(ppu_registers: Rc<RefCell<PpuRegisters>>) -> Self {
        Self {
            ppu_registers,
            ram: [0; 0x800],
            cartridge: Default::default(),
        }
    }

    pub fn connect_cartridge(&mut self, cartridge: Option<Rc<RefCell<Cartridge>>>) {
        self.cartridge = cartridge;
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.cartridge
            .as_ref()
            .unwrap()
            .borrow()
            .cpu_read(address)
            .unwrap_or_else(|| match address {
                0x0000..=0x1FFF => self.ram[address as usize & 0x7FF],
                0x2000..=0x3FFF => self.ppu_registers.borrow().read(address & 0x7),
                0x4000..=0x401F => todo!("APU/IO registers"),
                0x4020..=0xFFFF => todo!("..."),
            })
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        self.cartridge
            .as_ref()
            .unwrap()
            .borrow_mut()
            .cpu_write(address, data)
            .unwrap_or_else(|| {
                match address {
                    0x0000..=0x1FFF => self.ram[address as usize & 0x7FF] = data,
                    0x2000..=0x3FFF => self.ppu_registers.borrow_mut().write(address & 0x7, data),
                    0x4000..=0x401F => todo!("APU/IO registers"),
                    0x4020..=0xFFFF => todo!("..."),
                };
            });
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address + 1)])
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, value.to_le_bytes()[0]);
        self.write_u8(address + 1, value.to_le_bytes()[1]);
    }
}
