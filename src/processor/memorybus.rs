#![allow(dead_code)]
#[derive(Debug)]
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
    pub pc: usize,
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; 0xFFFF],
            pc: 0,
        }
    }

    pub fn fetch_next_instruction(&mut self) -> u8 {
        self.fetch_byte(0)
    }

    pub fn fetch_word(self: &mut MemoryBus) -> u16 {
        let value = self.memory[self.pc] as u16;
        let value_2 = self.memory[self.pc + 1] as u16;
        self.pc += 2;
        (value_2 << 8) | value
    }

    pub fn set_word(self: &mut MemoryBus, value: u16) {
        let value_1: u8 = (value >> 8) as u8;
        let value_2: u8 = (value & 0xFF) as u8;
        self.memory[self.pc] = value_1;
        self.memory[self.pc + 1] = value_2;
    }

    pub fn fetch_byte(self: &mut MemoryBus, position: usize) -> u8 {
        let mut position = position;
        if position == 0 {
            position = self.pc;
            self.pc += 1;
        }
        self.memory[position]
    }

    pub fn set_byte(self: &mut MemoryBus, value: u8, position: usize) {
        let mut position = position;
        if position == 0 {
            position = self.pc;
        }
        self.memory[position] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fetch_word_from_memory() {
        let mut memory = MemoryBus::new();
        memory.pc = 0xFF;
        let save_pc = memory.pc;
        memory.memory[0xFF] = 0xab;
        memory.memory[0xFF + 1] = 0xa0;
        let res = memory.fetch_word();

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
    fn it_fetch_byte_from_memory() {
        let mut memory = MemoryBus::new();
        memory.pc = 0xFF;
        let save_pc = memory.pc;
        memory.memory[0xFF] = 0xab;
        let res = memory.fetch_byte(0);

        assert_eq!(res, 0xab);
        assert_eq!(memory.pc, save_pc + 1);
    }

    #[test]
    fn it_set_byte_from_memory() {
        let mut memory = MemoryBus::new();
        memory.set_byte(0xf8, 0xFF);

        assert_eq!(memory.memory[0xFF], 0xf8);
    }

    #[test]
    fn it_set_byte_from_memory_from_pc() {
        let mut memory = MemoryBus::new();
        memory.pc = 0xFF;
        memory.set_byte(0xf8, 0);

        assert_eq!(memory.memory[0xFF], 0xf8);
    }
}
