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

    // FIXME: CPU init code should be properly implemented
    cpu.cycle = 7;

    for line in include_str!("nestest.log").lines() {
        assert_nestest_log_line(line, &cpu);
        cpu.next_operation(&mut memory);
    }
}

fn assert_nestest_log_line(line: &str, cpu: &Cpu) {
    let program_counter = u16::from_str_radix(&line[0..4], 16).unwrap();
    let accumulator: u8 = u8::from_str_radix(&line[50..52], 16).unwrap();
    let index_x: u8 = u8::from_str_radix(&line[55..57], 16).unwrap();
    let index_y: u8 = u8::from_str_radix(&line[60..62], 16).unwrap();
    let cycle: usize = line[90..].parse().unwrap();

    assert_eq!(program_counter, cpu.registers.program_counter);
    assert_eq!(accumulator, cpu.registers.accumulator);
    assert_eq!(index_x, cpu.registers.index_x);
    assert_eq!(index_y, cpu.registers.index_y);
    assert_eq!(cycle, cpu.cycle);

    // TODO: Stack pointer and PPU
}
