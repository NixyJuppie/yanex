use dioxus::prelude::*;
use yanex_core::Cpu;

pub fn Registers(cx: Scope) -> Element {
    let cpu = use_shared_state::<Cpu>(cx).unwrap();

    cx.render(rsx! {
        h2 { "Registers"},
        h3 { "Accumulator"},
        BitSwitch {
            value: cpu.read().registers.accumulator,
            set_value: move |v| {cpu.write().registers.accumulator = v;}
        }

        h3 { "Index X"},
        BitSwitch {
            value: cpu.read().registers.index_x,
            set_value: move |v| {cpu.write().registers.index_x = v;}
        }

        h3 { "Index Y"},
        BitSwitch {
            value: cpu.read().registers.index_y,
            set_value: move |v| {cpu.write().registers.index_y = v;}
        }
    })
}

#[inline_props]
pub fn BitSwitch<F>(cx: Scope, value: u8, set_value: F) -> Element
where
    F: Fn(u8),
{
    cx.render(rsx! {
        for i in 0..8 {
            button {
                background_color: if value & 0b00000001 << i == 0 {
                    "red"
                } else {
                    "green"
                },
                onclick: move |_| {
                    (set_value)(value ^ 0b00000001 << i)
                },
                "Bit {i}"
            }
        }
    })
}
