mod cpu_registers;

use crate::cpu::cpu_registers::CpuRegistersModel;
use relm4::adw::prelude::*;
use relm4::{
    adw, component, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use yanex_core::{Cpu, CpuRegisters};

#[derive(Debug)]
pub struct CpuModel {
    cpu: Cpu,
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
        let registers = CpuRegistersModel::builder()
            .launch(cpu.registers.clone())
            .forward(sender.input_sender(), |registers| {
                CpuInput::UpdateRegisters(registers)
            });
        let model = CpuModel { cpu, registers };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        adw::PreferencesGroup {
            add = model.registers.widget(),
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            CpuInput::UpdateRegisters(registers) => self.cpu.registers = registers,
        }
    }
}
