use control::ControlView;
use dioxus::prelude::*;
use registers::RegistersView;

mod control;
mod registers;

#[inline_props]
pub fn CpuView(cx: Scope) -> Element {
    cx.render(rsx! {
        ControlView {}
        RegistersView {}
    })
}
