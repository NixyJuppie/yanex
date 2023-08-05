use crate::{Cartridge, Cpu, CpuMemory};

#[test]
fn nestest() {
    let mut cpu = Cpu::default();
    let mut memory = CpuMemory::default();

    let cartridge: Cartridge = include_bytes!("nestest.nes").to_vec().try_into().unwrap();
    memory.connect_cartridge(cartridge);
    cpu.reset(&memory);

    // Start of non-interactive test
    cpu.registers.program_counter = 0xC000;

    for line in include_str!("nestest.log").lines() {
        cpu.next_operation(&mut memory);
        assert_eq!(line, generate_nestest_log_line(&cpu));
    }
}

fn generate_nestest_log_line(_cpu: &Cpu) -> &str {
    let foo = 123;
    todo!()
}
