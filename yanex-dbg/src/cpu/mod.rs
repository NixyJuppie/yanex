use bit_switch::BitSwitch;
use dioxus::prelude::*;
use yanex_core::Cpu;

mod bit_switch;

#[inline_props]
pub fn CpuView<'a>(cx: Scope, cpu: &'a UseRef<Cpu>) -> Element {
    cx.render(rsx! {
        h2 { "Registers" }
        h3 { "Accumulator" }
        BitSwitch {
            value: cpu.read().registers.accumulator,
            set_value: move |v| {
                cpu.write().registers.accumulator = v;
            }
        }

        h3 { "Index X" }
        BitSwitch {
            value: cpu.read().registers.index_x,
            set_value: move |v| {
                cpu.write().registers.index_x = v;
            }
        }

        h3 { "Index Y" }
        BitSwitch {
            value: cpu.read().registers.index_y,
            set_value: move |v| {
                cpu.write().registers.index_y = v;
            }
        }
    })
}
