#![allow(non_snake_case)]

mod cpu;

use crate::cpu::CpuView;
use dioxus::prelude::*;
use yanex_core::Opcode::{LdaImm, LdxAbs, NopImp};
use yanex_core::{Cpu, Memory, MemoryAccess};

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
    use_shared_state_provider(cx, Cpu::default);

    let mut memory = Memory::default();
    memory.write_opcode(0x0000, LdaImm);
    memory.write_u8(0x0001, 0x69);
    memory.write_opcode(0x0002, LdxAbs);
    memory.write_u8(0x0003, 0x00);
    memory.write_u8(0x0004, 0x03);
    for i in 0..100 {
        memory.write_opcode(0x0005 + i, NopImp);
    }
    use_shared_state_provider(cx, || memory);

    cx.render(rsx! { CpuView {} })
}
