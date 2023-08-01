use relm4::adw::prelude::*;
use relm4::{adw, component, gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug, Clone)]
pub struct BitNames {
    pub bit0: String,
    pub bit1: String,
    pub bit2: String,
    pub bit3: String,
    pub bit4: String,
    pub bit5: String,
    pub bit6: String,
    pub bit7: String,
}

impl Default for BitNames {
    fn default() -> Self {
        BitNames {
            bit0: "0".to_string(),
            bit1: "1".to_string(),
            bit2: "2".to_string(),
            bit3: "3".to_string(),
            bit4: "4".to_string(),
            bit5: "5".to_string(),
            bit6: "6".to_string(),
            bit7: "7".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct BitSwitchRowModel {
    value: u8,
    should_update_hex_value: bool,
    should_update_dec_value: bool,
    name: String,
    bit_names: BitNames,
}

#[derive(Debug)]
pub enum BitSwitchRowInput {
    BitFlipped(u8),
    HexValueSet(String),
    DecValueSet(String),
}

#[component(pub)]
impl SimpleComponent for BitSwitchRowModel {
    type Input = BitSwitchRowInput;
    type Output = u8;
    type Init = (u8, String, BitNames);

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BitSwitchRowModel {
            value: init.0,
            should_update_hex_value: true,
            should_update_dec_value: true,
            name: init.1,
            bit_names: init.2,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        adw::ExpanderRow {
            set_title: &model.name,
            #[watch]
            set_subtitle: &format!("0x{}", format_hex(model.value)),
            add_row = &adw::EntryRow {
                set_title: "Value (HEX)",
                #[track(model.should_update_hex_value)]
                set_text: &format_hex(model.value),
                set_show_apply_button: true,
                set_input_purpose: gtk::InputPurpose::Digits,
                connect_apply[sender] => move |r| sender.input(BitSwitchRowInput::HexValueSet(r.text().to_string()))
            },
            add_row = &adw::EntryRow {
                set_title: "Value (DEC)",
                #[track(model.should_update_dec_value)]
                set_text: &format_dec(model.value),
                set_show_apply_button: true,
                set_input_purpose: gtk::InputPurpose::Digits,
                connect_apply[sender] => move |r| sender.input(BitSwitchRowInput::DecValueSet(r.text().to_string()))
            },
            add_row = &adw::ActionRow {
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_margin_all: 5,
                    set_spacing: 5,
                    set_halign: gtk::Align::Center,
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit7,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 7) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(7)
                    },
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit6,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 6) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(6)
                    },
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit5,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 5) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(5)
                    },
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit4,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 4) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(4)
                    },
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit3,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 3) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(3)
                    },
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit2,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 2) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(2)
                    },
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit1,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 1) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(1)
                    },
                    gtk::Button {
                        #[watch]
                        set_label: &model.bit_names.bit0,
                        #[watch]
                        set_css_classes: if model.value & (0b_0000_0001 << 0) > 0 { &["suggested-action"] } else { &[] },
                        connect_clicked => BitSwitchRowInput::BitFlipped(0)
                    },
                }
            }
        }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        match input {
            BitSwitchRowInput::BitFlipped(bit) => {
                self.value ^= 0b_0000_0001 << bit;
                self.should_update_hex_value = true;
                self.should_update_dec_value = true;
                sender.output(self.value).unwrap();
            }
            BitSwitchRowInput::HexValueSet(value) => {
                self.should_update_hex_value = false;
                match u8::from_str_radix(&value, 16) {
                    Ok(value) => {
                        if self.value != value {
                            self.value = value;
                            self.should_update_dec_value = true;
                            sender.output(self.value).unwrap();
                        }
                    }
                    Err(error) => {
                        println!("{}: error {:?}", value, error)
                    }
                }
            }
            BitSwitchRowInput::DecValueSet(value) => {
                self.should_update_dec_value = false;
                match value.parse() {
                    Ok(value) => {
                        if self.value != value {
                            self.value = value;
                            self.should_update_hex_value = true;
                            sender.output(self.value).unwrap();
                        }
                    }
                    Err(error) => {
                        println!("{}: error {:?}", value, error)
                    }
                }
            }
        }
    }
}

fn format_hex(value: u8) -> String {
    format!("{:02X}", value).to_string()
}

fn format_dec(value: u8) -> String {
    format!("{:03}", value).to_string()
}
