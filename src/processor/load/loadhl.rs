use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loadhl_dispatch(&mut self, target: Target) {
        let position = self.registers.hl() as usize;
        match target {
            Target::A => self.memory.set_byte(self.registers.a, position),
            Target::B => self.memory.set_byte(self.registers.b, position),
            Target::C => self.memory.set_byte(self.registers.c, position),
            Target::D => self.memory.set_byte(self.registers.d, position),
            Target::E => self.memory.set_byte(self.registers.e, position),
            Target::H => self.memory.set_byte(self.registers.h, position),
            Target::L => self.memory.set_byte(self.registers.l, position),
            Target::HL => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        let position: usize = 0x11;
        cpu.registers.set_hl(position as u16);
        cpu.memory.set_byte(0x70, 0);

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte_at(position), 0x13);
    }

    #[test]
    fn it_should_move_a_to_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        let position: usize = 0x11;
        cpu.registers.set_hl(position as u16);
        cpu.memory.set_byte(0x77, 0);

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte_at(position), 0x13);
    }

    #[test]
    fn it_should_move_c_to_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        let position: usize = 0x11;
        cpu.registers.set_hl(position as u16);
        cpu.memory.set_byte(0x71, 0);

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte_at(position), 0x13);
    }

    #[test]
    fn it_should_move_d_to_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x72, 0);
        let position: usize = 0x11;
        cpu.registers.set_hl(position as u16);

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte_at(position), 0x13);
    }

    #[test]
    fn it_should_move_e_to_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x73, 0);
        let position: usize = 0x11;
        cpu.registers.set_hl(position as u16);

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte_at(position), 0x13);
    }

    #[test]
    fn it_should_move_h_to_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.registers.h = 0x15;
        cpu.memory.set_byte(0x74, 0);

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte_at(cpu.registers.hl() as usize), 0x15);
    }

    #[test]
    fn it_should_move_l_to_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.registers.h = 0x15;
        cpu.memory.set_byte(0x75, 0);

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte_at(cpu.registers.hl() as usize), 0x13);
    }
}
