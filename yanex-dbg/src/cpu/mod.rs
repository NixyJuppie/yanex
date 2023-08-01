mod cpu_control;
mod cpu_registers;

use crate::cpu::cpu_control::CpuControlModel;
use crate::cpu::cpu_registers::CpuRegistersModel;
use relm4::adw::prelude::*;
use relm4::{
    component, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use yanex_core::{Cpu, CpuRegisters};

#[derive(Debug)]
pub struct CpuModel {
    cpu: Cpu,
    control: Controller<CpuControlModel>,
    registers: Controller<CpuRegistersModel>,
}

#[derive(Debug)]
pub enum CpuInput {
    UpdateRegisters(CpuRegisters),
}

#[component(pub)]
impl SimpleComponent for CpuModel {
    type Input = CpuInput;
    type Output = ();
    type Init = Cpu;

    fn init(
        cpu: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let control = CpuControlModel::builder().launch(()).detach();
        let registers = CpuRegistersModel::builder()
            .launch(cpu.registers.clone())
            .forward(sender.input_sender(), |registers| {
                CpuInput::UpdateRegisters(registers)
            });
        let model = CpuModel {
            cpu,
            control,
            registers,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        gtk::ScrolledWindow {
            set_min_content_width: 400,
            set_min_content_height: 400,
            gtk::FlowBox {
                set_selection_mode: gtk::SelectionMode::None, // Why this is not by default...
                set_orientation: gtk::Orientation::Horizontal,
                set_max_children_per_line: 2,
                append: model.control.widget(),
                append: model.registers.widget()
            }
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            CpuInput::UpdateRegisters(registers) => self.cpu.registers = registers,
        }
    }
}
