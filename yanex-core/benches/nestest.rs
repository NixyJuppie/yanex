use criterion::{criterion_group, criterion_main, Criterion};
use yanex_core::cartridge::Cartridge;
use yanex_core::cpu::{Cpu, CpuMemory};

fn nestest_benchmark(c: &mut Criterion) {
    let cartridge: Cartridge = include_bytes!("nestest.nes").to_vec().try_into().unwrap();

    let mut memory = CpuMemory::default();
    memory.connect_cartridge(&cartridge);

    let mut cpu = Cpu::default();
    cpu.reset(&mut memory);

    c.bench_function("nestest_legal 1s", |b| {
        b.iter(|| {
            nestest_legal(
                1_789_773, // 1.789773 MHz (~559 ns per cycle)
                cpu.clone(),
                memory.clone(),
            )
        })
    });
}

fn nestest_legal(cycles: usize, mut cpu: Cpu, mut memory: CpuMemory) {
    for i in 0..cycles {
        if i % 14572 == 0 {
            cpu.reset(&mut memory);
            cpu.registers.program_counter = 0xC000;
        }

        cpu.next_cycle(&mut memory);
    }
}

criterion_group!(benches, nestest_benchmark);
criterion_main!(benches);
