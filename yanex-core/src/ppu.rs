mod memory;

pub use memory::PpuMemory;

#[derive(Debug, Clone, Default)]
pub struct Ppu {}

impl Ppu {
    pub fn reset(&mut self, _memory: &mut PpuMemory) {
        // todo!()
    }

    pub fn next_cycle(&mut self, _memory: &mut PpuMemory) {
        // todo!()
    }
}
