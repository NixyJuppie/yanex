use yanex_core::{Cpu, Memory, MemoryAccess, Opcode};

fn main() {
    let mut memory = Memory::default();
    memory.write_u8(0, Opcode::LdaImm as u8);
    memory.write_u8(1, 0x69);
    memory.write_u8(2, Opcode::LdaAbs as u8);
    memory.write_u16(3, 5);
    memory.write_u8(5, 0x42);

    let mut cpu = Cpu::default();
    cpu.execute_operation(&mut memory, &mut None);
    assert_eq!(cpu.registers.accumulator, 0x69);
    cpu.execute_operation(&mut memory, &mut None);
    assert_eq!(cpu.registers.accumulator, 0x42);
}
