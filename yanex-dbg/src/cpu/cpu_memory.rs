use relm4::adw::prelude::*;
use relm4::component::{AsyncComponent, AsyncComponentParts};
use relm4::{adw, component, gtk, AsyncComponentSender, RelmWidgetExt};
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use yanex_core::{CpuMemory, INes};

#[derive(Debug)]
pub struct CpuMemoryModel {
    memory: Rc<RefCell<CpuMemory>>,
}

#[derive(Debug)]
pub enum CpuMemoryInput {
    LoadCartridge,
    CartridgeLoaded(Vec<u8>),
}

#[component(pub, async)]
impl AsyncComponent for CpuMemoryModel {
    type Input = CpuMemoryInput;
    type Output = ();
    type Init = Rc<RefCell<CpuMemory>>;
    type CommandOutput = ();

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = CpuMemoryModel { memory: init };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    view! {
        adw::PreferencesGroup {
            set_margin_all: 20,
            set_title: "Memory",
            add = &adw::ActionRow {
                set_title: "Cartridge",
                add_suffix = &gtk::Button {
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::Center,
                    set_label: "Load",
                    connect_clicked => CpuMemoryInput::LoadCartridge
                }
            }
        }
    }

    async fn update(
        &mut self,
        input: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match input {
            CpuMemoryInput::LoadCartridge => {
                let dialog = gtk::FileDialog::builder().build();
                if let Ok(file) = dialog.open_future(root.toplevel_window().as_ref()).await {
                    if let Ok(stream) = file.read_future(Default::default()).await {
                        let mut data = Vec::<u8>::new();
                        if stream.into_read().read_to_end(&mut data).is_ok() {
                            sender.input(CpuMemoryInput::CartridgeLoaded(data))
                        }
                    }
                }
            }
            CpuMemoryInput::CartridgeLoaded(data) => {
                let ines: INes = data.into();
                self.memory.borrow_mut().connect_cartridge(ines.into());
            }
        }
    }
}
