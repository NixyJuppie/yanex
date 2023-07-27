use dioxus::prelude::*;

#[inline_props]
pub fn BitSwitch<F>(cx: Scope, value: u8, set_value: F) -> Element
where
    F: Fn(u8),
{
    cx.render(rsx! {
        div { class: "btn-group",
            for i in 0..8 {
                button {
                    class: if value & 0b0000_0001 << i > 0 {
                        "btn btn-solid-primary"
                    } else {
                        "btn btn-warning"
                    },
                    onclick: move |_| {
                        (set_value)(value ^ 0b00000001 << i)
                    },
                    "{i}"
                }
            }
        }
    })
}
