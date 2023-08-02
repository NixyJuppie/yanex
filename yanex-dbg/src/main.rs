mod app;
mod components;
mod cpu;

use app::AppModel;
use relm4::RelmApp;
use std::cell::RefCell;
use std::rc::Rc;
use yanex_core::{Cpu, CpuMemory, Ppu, PpuMemory};

#[derive(Debug, Default)]
pub struct Nes {
    pub cpu: Rc<RefCell<Cpu>>,
    pub cpu_memory: Rc<RefCell<CpuMemory>>,
    pub ppu: Rc<RefCell<Ppu>>,
    pub ppu_memory: Rc<RefCell<PpuMemory>>,
}

fn main() {
    let app = RelmApp::new("org.nixyjuppie.yanexdbg");
    app.run::<AppModel>(("Yanex Debugger".to_string(), Nes::default()));
}
