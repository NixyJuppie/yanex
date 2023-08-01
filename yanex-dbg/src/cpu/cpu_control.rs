use relm4::adw::prelude::*;
use relm4::{adw, component, gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
pub struct CpuControlModel {
    // TODO
}

#[derive(Debug)]
pub enum CpuControlInput {
    // TODO
}

#[component(pub)]
impl SimpleComponent for CpuControlModel {
    type Input = CpuControlInput;
    type Output = ();
    // TODO
    type Init = (); // TODO

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CpuControlModel {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        adw::Clamp {
            set_margin_all: 20,
            set_maximum_size: 350,
            adw::PreferencesGroup {
                set_title: "Control",
                add = &adw::ActionRow {
                    set_title: "Cycle",
                    set_subtitle: "Current: 2137",
                    add_suffix = &gtk::Button {
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["destructive-action"],
                        set_label: "Next",
                    },
                },
                add = &adw::ActionRow {
                    set_title: "Operation",
                    set_subtitle: "Current: LoadAccumulator",
                    add_suffix = &gtk::Button {
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["destructive-action"],
                        set_label: "Next",
                    },
                },
            },
        }
    }
}
