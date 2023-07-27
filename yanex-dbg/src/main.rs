#![allow(non_snake_case)]

mod cpu;

use crate::cpu::CpuView;
use dioxus::prelude::*;
use yanex_core::Cpu;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    if cfg!(debug_assertions) {
        cx.render(rsx! {
            script { src: "https://livejs.com/live.js" }
            MainView {}
        })
    } else {
        cx.render(rsx! { MainView {} })
    }
}

fn MainView(cx: Scope) -> Element {
    let mut cpu = Cpu::default();
    cpu.registers.accumulator = 255;
    cpu.registers.index_x = 0;
    cpu.registers.index_y = 0b10101010;
    let cpu = use_ref(cx, || cpu);

    cx.render(rsx! { CpuView { cpu: cpu } })
}
