use crate::Opcode;

pub trait MemoryAccess {
    fn read_u8(&self, address: u16) -> u8;
    fn write_u8(&mut self, address: u16, value: u8);

    fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_u8(address), self.read_u8(address + 1)])
    }

    fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, value.to_le_bytes()[0]);
        self.write_u8(address + 1, value.to_le_bytes()[1]);
    }

    fn write_opcode(&mut self, address: u16, opcode: Opcode) {
        self.write_u8(address, opcode as u8);
    }
}
