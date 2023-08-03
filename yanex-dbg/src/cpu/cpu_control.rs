use relm4::adw::prelude::*;
use relm4::{adw, component, gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use std::cell::RefCell;
use std::rc::Rc;
use yanex_core::{Cpu, CpuMemory};

#[derive(Debug)]
pub struct CpuControlModel {
    cpu: Rc<RefCell<Cpu>>,
    memory: Rc<RefCell<CpuMemory>>,
}

#[derive(Debug)]
pub enum CpuControlInput {
    NextCycle,
    NextOperation,
    Reset,
}

#[component(pub)]
impl SimpleComponent for CpuControlModel {
    type Input = CpuControlInput;
    type Output = ();
    type Init = (Rc<RefCell<Cpu>>, Rc<RefCell<CpuMemory>>);

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CpuControlModel {
            cpu: init.0,
            memory: init.1,
        };
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
                    #[watch]
                    set_subtitle: &format!("Current: {}", model.cpu.borrow().cycle),
                    add_suffix = &gtk::Button {
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["suggested-action"],
                        set_label: "Next",
                        connect_clicked => CpuControlInput::NextCycle
                    },
                },
                add = &adw::ActionRow {
                    set_title: "Operation",
                    #[watch]
                    set_subtitle: &format!("Current: {}", model.cpu.borrow().state),
                    add_suffix = &gtk::Button {
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["suggested-action"],
                        set_label: "Next",
                        connect_clicked => CpuControlInput::NextOperation
                    },
                },
                add = &adw::ActionRow {
                    set_title: "Reset",
                    set_subtitle: "Resets the CPU",
                    add_suffix = &gtk::Button {
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["destructive-action"],
                        set_label: "Reset",
                        connect_clicked => CpuControlInput::Reset
                    },
                },
            },
        }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>) {
        match input {
            CpuControlInput::NextCycle => self
                .cpu
                .borrow_mut()
                .next_cycle(&mut self.memory.borrow_mut()),
            CpuControlInput::NextOperation => self
                .cpu
                .borrow_mut()
                .next_operation(&mut self.memory.borrow_mut()),
            CpuControlInput::Reset => self.cpu.borrow_mut().reset(&self.memory.borrow()),
        }
    }
}
