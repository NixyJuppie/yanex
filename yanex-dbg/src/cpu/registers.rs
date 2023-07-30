use dioxus::prelude::*;
use std::num::ParseIntError;
use yanex_core::{Cpu, CpuStatus};

#[inline_props]
pub fn RegistersView(cx: Scope) -> Element {
    let cpu = use_shared_state::<Cpu>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "p-4",
            h1 {
                class: "text-center text-2xl text-content-1 font-extrabold",
                "Registers"
            },

            div {
                class: "flex flex-wrap justify-evenly",
                RegisterView {
                    name: "Status",
                    bit_names: ["C", "Z", "I", "D", "B", "_", "V", "N"]
                    value: cpu.read().registers.status.bits(),
                    set_value: move |v| {
                        cpu.write().registers.status = CpuStatus::from_bits_truncate(v);
                    }
                },

                RegisterView {
                    name: "Accumulator",
                    bit_names: ["0", "1", "2", "3", "4", "5", "6", "7"]
                    value: cpu.read().registers.accumulator,
                    set_value: move |v| {
                        cpu.write().registers.accumulator = v;
                    }
                }

                RegisterView {
                    name: "X Index",
                    bit_names: ["0", "1", "2", "3", "4", "5", "6", "7"]
                    value: cpu.read().registers.index_x,
                    set_value: move |v| {
                        cpu.write().registers.index_x = v;
                    }
                }

                RegisterView {
                    name: "Y Index",
                    bit_names: ["0", "1", "2", "3", "4", "5", "6", "7"]
                    value: cpu.read().registers.index_y,
                    set_value: move |v| {
                        cpu.write().registers.index_y = v;
                    }
                }
            }
        }
    })
}

#[inline_props]
fn RegisterView<'a, F>(
    cx: Scope,
    name: &'a str,
    bit_names: [&'a str; 8],
    value: u8,
    set_value: F,
) -> Element
where
    F: Fn(u8),
{
    let is_valid = use_state(cx, || true);
    let input_value = use_state(cx, || format!("0x{:02X}", value));

    use_effect(cx, value, |value| {
        to_owned![input_value];
        async move {
            input_value.set(format!("0x{:02X}", value));
        }
    });

    cx.render(rsx! {
        div {
            padding: "20px",
            div {
                class: "card",
                div {
                    class: "card-body",
                    h2 {
                        class: "card-header",
                        *name

                        input {
                            class: if *is_valid.get() { "input w-20" } else { "input w-20 input-error" },
                            value: "{input_value}",
                            onchange: move |e| {
                                let result = parse_u8(e.value.clone());
                                input_value.set(e.value.clone());
                                if let Ok(val) = result {
                                    is_valid.set(true);
                                    (set_value)(val)
                                } else {
                                    is_valid.set(false);
                                }
                            }
                        }
                    },
                    div {
                        class: "card-footer",
                        div { class: "btn-group",
                            for i in 0..8 {
                                button {
                                    class: if value & 0b_0000_0001 << i > 0 {
                                        "btn btn-sm btn-solid-success"
                                    } else {
                                        "btn btn-sm btn-solid-error"
                                    },
                                    onclick: move |_| {
                                        let new_value = value ^ 0b_0000_0001 << i;
                                        (set_value)(new_value);
                                        input_value.set(format!("0x{:02X}", new_value));
                                        is_valid.set(true);
                                    },
                                    "{bit_names[i]}"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

fn parse_u8(value: String) -> Result<u8, ParseIntError> {
    let value = value.trim_start_matches("0x");
    u8::from_str_radix(value, 16)
}
