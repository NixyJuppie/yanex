use crate::components::{BitNames, BitSwitchRowModel};

use relm4::adw::prelude::*;
use relm4::{
    adw, component, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    RelmWidgetExt, SimpleComponent,
};
use yanex_core::{CpuRegisters, CpuStatus};

#[derive(Debug)]
pub struct CpuRegistersModel {
    registers: CpuRegisters,
    status: Controller<BitSwitchRowModel>,
    accumulator: Controller<BitSwitchRowModel>,
    index_x: Controller<BitSwitchRowModel>,
    index_y: Controller<BitSwitchRowModel>,
}

#[derive(Debug)]
pub enum CpuRegistersInput {
    Status(CpuStatus),
    Accumulator(u8),
    IndexX(u8),
    IndexY(u8),
}

#[component(pub)]
impl SimpleComponent for CpuRegistersModel {
    type Input = CpuRegistersInput;
    type Output = CpuRegisters;
    type Init = CpuRegisters;

    fn init(
        registers: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let status = BitSwitchRowModel::builder()
            .launch((
                registers.status.bits(),
                "Status".to_string(),
                BitNames {
                    bit0: "C".to_string(),
                    bit1: "Z".to_string(),
                    bit2: "I".to_string(),
                    bit3: "D".to_string(),
                    bit4: "B".to_string(),
                    bit5: "_".to_string(),
                    bit6: "V".to_string(),
                    bit7: "N".to_string(),
                },
            ))
            .forward(sender.input_sender(), |value| {
                CpuRegistersInput::Status(CpuStatus::from_bits(value).unwrap())
            });
        let accumulator = BitSwitchRowModel::builder()
            .launch((
                registers.accumulator,
                "Accumulator".to_string(),
                BitNames::default(),
            ))
            .forward(sender.input_sender(), |value| {
                CpuRegistersInput::Accumulator(value)
            });
        let index_x = BitSwitchRowModel::builder()
            .launch((
                registers.index_x,
                "Index X".to_string(),
                BitNames::default(),
            ))
            .forward(sender.input_sender(), |value| {
                CpuRegistersInput::IndexX(value)
            });
        let index_y = BitSwitchRowModel::builder()
            .launch((
                registers.index_y,
                "Index Y".to_string(),
                BitNames::default(),
            ))
            .forward(sender.input_sender(), |value| {
                CpuRegistersInput::IndexY(value)
            });

        let model = CpuRegistersModel {
            registers,
            status,
            accumulator,
            index_x,
            index_y,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        adw::Clamp {
            set_margin_all: 20,
            set_maximum_size: 350,
            adw::PreferencesGroup {
                set_title: "Registers",
                add = model.status.widget(),
                add = model.accumulator.widget(),
                add = model.index_x.widget(),
                add = model.index_y.widget(),
            }
        }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        match input {
            CpuRegistersInput::Status(value) => {
                self.registers.status = value;
            }
            CpuRegistersInput::Accumulator(value) => {
                self.registers.accumulator = value;
            }
            CpuRegistersInput::IndexX(value) => {
                self.registers.index_x = value;
            }
            CpuRegistersInput::IndexY(value) => {
                self.registers.index_y = value;
            }
        }

        sender.output(self.registers.clone()).unwrap();
    }
}
