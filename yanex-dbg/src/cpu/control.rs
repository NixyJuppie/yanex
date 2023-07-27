use dioxus::prelude::*;
use yanex_core::{Cpu, Memory, Operation};

#[inline_props]
pub fn ControlView(cx: Scope) -> Element {
    let cpu = use_shared_state::<Cpu>(cx).unwrap();
    let memory = use_shared_state::<Memory>(cx).unwrap();
    let state = use_ref(cx, || None::<Operation>);

    cx.render(rsx! {
        div {
            class: "p-4",
            h1 {
                class: "text-center text-2xl text-content-1 font-extrabold",
                "Emulation"
            },
            h2 {
                class: "text-center text-2 p-1",
                p { "Cycle: {cpu.read().cycle}" },
                p { "Operation: {state.read():?}" },
            },
            div {
                class: "flex justify-center gap-2",
                button {
                    class: "btn btn-primary",
                    onclick: |_| {
                        cpu.write().next_cycle(&mut memory.write(), &mut state.write())
                    },
                    "Next Cycle"
                },
                button {
                    class: "btn btn-success",
                    onclick: |_| {
                        cpu.write().next_operation(&mut memory.write(), &mut state.write())
                    },
                    "Next Operation"
                },
                button {
                    class: "btn btn-error",
                    onclick: |_| {
                        // TODO
                        *cpu.write() = Cpu::default();
                        state.set(None);
                    },
                    "Reset"
                }
            }
        }
    })
}
