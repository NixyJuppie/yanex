use criterion::{criterion_group, criterion_main, Criterion};
use yanex_core::{Cartridge, Nes};

fn nestest_benchmark(c: &mut Criterion) {
    let mut nes = Nes::default();
    let cartridge: Cartridge = include_bytes!("nestest.nes").to_vec().try_into().unwrap();
    nes.insert_cartridge(Some(cartridge));
    nes.reset();

    c.bench_function("nestest_legal 1s", |b| {
        b.iter(|| {
            nestest_legal(
                1_789_773, // 1.789773 MHz (~559 ns per cycle)
                nes.clone(),
            )
        })
    });
}

fn nestest_legal(cycles: usize, mut nes: Nes) {
    for i in 0..cycles {
        // Cycle on end of non-interactive test
        if i % 14572 == 0 {
            nes.reset();
            // Start of non-interactive test
            nes.cpu.registers.program_counter = 0xC000;
        }

        nes.next();
    }
}

criterion_group!(benches, nestest_benchmark);
criterion_main!(benches);
