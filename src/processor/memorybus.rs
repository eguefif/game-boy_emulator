#![allow(dead_code)]
#[derive(Debug)]
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
    pub pc: usize,
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0xFC; 0xFFFF],
            pc: 0,
        }
    }

    pub fn fetch_next_instruction(&mut self) -> u8 {
        self.fetch_next_byte()
    }

    fn move_pc_by(&mut self, to_add: usize) {
        let value = self.pc;
        self.pc = value.wrapping_add(to_add);
    }

    pub fn fetch_next_word(self: &mut MemoryBus) -> u16 {
        let low = self.memory[self.pc] as u16;
        let high = self.memory[self.pc + 1] as u16;
        self.move_pc_by(2);
        (high << 8) | low
    }

    pub fn fetch_word_at(self: &mut MemoryBus, position: usize) -> u16 {
        let low = self.memory[position] as u16;
        let high = self.memory[position + 1] as u16;
        (high << 8) | low
    }

    pub fn set_word(self: &mut MemoryBus, value: u16) {
        let value_1: u8 = (value >> 8) as u8;
        let value_2: u8 = (value & 0xFF) as u8;
        self.memory[self.pc] = value_1;
        self.memory[self.pc + 1] = value_2;
    }

    pub fn fetch_byte_at(self: &mut MemoryBus, position: usize) -> u8 {
        self.memory[position]
    }

    pub fn fetch_next_byte(self: &mut MemoryBus) -> u8 {
        let position = self.pc;
        self.move_pc_by(1);
        self.memory[position]
    }

    pub fn set_byte(self: &mut MemoryBus, value: u8, position: usize) {
        self.memory[position] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fetch_next_word_from_memory() {
        let mut memory = MemoryBus::new();
        memory.pc = 0xFF;
        let save_pc = memory.pc;
        memory.memory[0xFF] = 0xab;
        memory.memory[0xFF + 1] = 0xa0;
        let res = memory.fetch_next_word();

        assert_eq!(res, 0xa0ab);
        assert_eq!(memory.pc, save_pc + 2);
    }

    #[test]
    fn it_set_word_from_memory() {
        let mut memory = MemoryBus::new();
        memory.pc = 0xFF;
        memory.set_word(0xf80a);

        assert_eq!(memory.memory[memory.pc], 0xf8);
        assert_eq!(memory.memory[memory.pc + 1], 0x0a);
    }

    #[test]
    fn it_fetch_byte_at_from_memory() {
        let mut memory = MemoryBus::new();
        memory.memory[0x15] = 0xab;
        let res = memory.fetch_byte_at(0x15);

        assert_eq!(res, 0xab);
    }

    #[test]
    fn it_fetch_next_byte_from_memory() {
        let mut memory = MemoryBus::new();
        memory.pc = 0xab;
        memory.memory[0xab] = 0x1e;
        let res = memory.fetch_next_byte();

        assert_eq!(res, 0x1e);
        assert_eq!(memory.pc, 0xab + 1);
    }

    #[test]
    fn it_set_byte_from_memory() {
        let mut memory = MemoryBus::new();
        memory.set_byte(0xf8, 0xFF);

        assert_eq!(memory.memory[0xFF], 0xf8);
    }
}
