mod cpu_control;
mod cpu_memory;
mod cpu_registers;

use cpu_control::CpuControlModel;
use cpu_memory::CpuMemoryModel;
use cpu_registers::CpuRegistersModel;
use relm4::adw::prelude::*;
use relm4::component::{AsyncComponent, AsyncComponentController, AsyncController};
use relm4::{
    component, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use std::cell::RefCell;
use std::rc::Rc;
use yanex_core::{Cpu, CpuMemory, CpuRegisters};

#[derive(Debug)]
pub struct CpuModel {
    cpu: Rc<RefCell<Cpu>>,
    control_component: Controller<CpuControlModel>,
    registers_component: Controller<CpuRegistersModel>,
    memory_component: AsyncController<CpuMemoryModel>,
}

#[derive(Debug)]
pub enum CpuInput {
    UpdateRegisters(CpuRegisters),
}

#[component(pub)]
impl SimpleComponent for CpuModel {
    type Input = CpuInput;
    type Output = ();
    type Init = (Rc<RefCell<Cpu>>, Rc<RefCell<CpuMemory>>);

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let control_component = CpuControlModel::builder().launch(init.clone()).detach();
        let registers_component = CpuRegistersModel::builder()
            .launch(init.0.borrow().registers.clone())
            .forward(sender.input_sender(), |registers| {
                CpuInput::UpdateRegisters(registers)
            });
        let memory_component = CpuMemoryModel::builder().launch(init.1.clone()).detach();
        let model = CpuModel {
            cpu: init.0,
            control_component,
            registers_component,
            memory_component,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
                gtk::FlowBox {
                    set_selection_mode: gtk::SelectionMode::None, // Why this is not by default...
                    set_orientation: gtk::Orientation::Horizontal,
                    set_max_children_per_line: 2,
                    append: model.control_component.widget(),
                    append: model.registers_component.widget(),
                },
            append: model.memory_component.widget()
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            CpuInput::UpdateRegisters(registers) => self.cpu.borrow_mut().registers = registers,
        }
    }
}
