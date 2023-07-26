#![allow(non_snake_case)]

mod registers;

use dioxus::prelude::*;
use registers::Registers;
use yanex_core::Cpu;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let mut cpu = Cpu::default();
    cpu.registers.accumulator = 0b1010_1011;
    cpu.registers.index_x = 0b1111_0000;
    cpu.registers.index_y = 0b0000_1111;
    use_shared_state_provider(cx, || cpu);

    cx.render(rsx! {
        h1 { "CPU" }

        Registers { }
    })
}
