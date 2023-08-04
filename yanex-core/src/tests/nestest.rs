use crate::{Cartridge, Cpu, CpuMemory};

#[test]
fn nestest() {
    let mut cpu = Cpu::default();
    let mut memory = CpuMemory::default();

    let cartridge: Cartridge = include_bytes!("nestest.nes").to_vec().try_into().unwrap();
    memory.connect_cartridge(cartridge);
    cpu.reset(&memory);

    cpu.registers.program_counter = 0xC000;

    cpu.next_operation(&mut memory);
    assert_eq!(cpu.cycle, 7);

    // TODO: Generate log and compare with `nestest.log`
}
