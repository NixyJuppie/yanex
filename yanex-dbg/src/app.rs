use crate::cpu::CpuModel;
use crate::Nes;

use adw::prelude::*;
use relm4::{
    adw, component, gtk, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, SimpleComponent,
};

#[derive(Debug)]
pub struct AppModel {
    title: String,
    cpu_component: Controller<CpuModel>,
}

#[component(pub)]
impl SimpleComponent for AppModel {
    type Input = ();
    type Output = ();
    type Init = (String, Nes);

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cpu_component = CpuModel::builder()
            .launch((init.1.cpu.clone(), init.1.cpu_memory.clone()))
            .detach();

        let model = AppModel {
            title: init.0,
            cpu_component,
        };

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
                    // add_titled_with_icon[Some("play"), "Play", "applications-games-symbolic"] = &gtk::Box {},
                    add_titled_with_icon[Some("cpu"), "CPU", "edit-select-all-symbolic"] = model.cpu_component.widget(),
                    add_titled_with_icon[Some("ppu"), "PPU", "video-display-symbolic"] = &gtk::Box {},
                    add_titled_with_icon[Some("apu"), "APU", "audio-speakers-symbolic"] = &gtk::Box {},
                },
                #[name(bar)]
                adw::ViewSwitcherBar {
                    set_stack: Some(&stack),
                }
            }
        }
    }
}
