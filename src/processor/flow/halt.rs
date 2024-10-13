use crate::processor::cpu::Cpu;

impl Cpu {
    pub fn halt(&mut self) {
        self.pause = true;
    }

    pub fn is_halted(&self) -> bool {
        self.pause
    }
}
