mod components;
mod cpu;

use cpu::CpuModel;
use relm4::adw::prelude::*;
use relm4::{
    adw, component, gtk, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, RelmApp, SimpleComponent,
};
use yanex_core::Cpu;

#[derive(Debug)]
struct AppModel {
    title: String,
    cpu: Controller<CpuModel>,
}

#[component]
impl SimpleComponent for AppModel {
    type Input = ();
    type Output = ();
    type Init = String;

    fn init(
        title: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cpu = CpuModel::builder().launch(Cpu::default()).detach();
        let model = AppModel { title, cpu };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    view! {
        adw::Window {
            set_title: Some(&model.title),
            set_default_width: 800,
            set_default_height: 600,
            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::ViewSwitcherTitle {
                        set_title: &model.title,
                        set_stack: Some(&stack),
                        #[chain(build())]
                        bind_property: ("title-visible", &bar, "reveal")
                    },
                },
                #[name(stack)]
                adw::ViewStack {
                    set_vexpand: true,
                    set_hexpand: true,
                    add_titled_with_icon[Some("cpu"), "CPU", "edit-select-all-symbolic"] = model.cpu.widget(),
                    add_titled_with_icon[Some("ppu"), "PPU", "video-display-symbolic"] = &gtk::Box {},
                    add_titled_with_icon[Some("apu"), "APU", "audio-speakers-symbolic"] = &gtk::Box {},
                    add_titled_with_icon[Some("memory"), "Memory", "media-flash-symbolic"] = &gtk::Box {},

                },
                #[name(bar)]
                adw::ViewSwitcherBar {
                    set_stack: Some(&stack),
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("org.nixyjuppie.yanexdbg");
    app.run::<AppModel>("Yanex Debugger".to_string());
}
