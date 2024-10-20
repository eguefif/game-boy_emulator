use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loadb_dispatch(&mut self, target: Target) {
        match target {
            Target::A => self.registers.b = self.registers.a,
            Target::B => {}
            Target::C => self.registers.b = self.registers.c,
            Target::D => self.registers.b = self.registers.d,
            Target::E => self.registers.b = self.registers.e,
            Target::H => self.registers.b = self.registers.h,
            Target::L => self.registers.b = self.registers.l,
            Target::HL => {
                let position = self.registers.hl() as usize;
                self.registers.b = self.memory.fetch_byte_at(position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_b() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        cpu.memory.set_byte(0x40, 0);

        cpu.run();

        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_a_to_b() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        cpu.memory.set_byte(0x47, 0);

        cpu.run();

        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_c_to_b() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        cpu.memory.set_byte(0x41, 0);

        cpu.run();

        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_d_to_b() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x42, 0);

        cpu.run();

        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_e_to_b() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x43, 0);

        cpu.run();

        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_h_to_b() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x13;
        cpu.memory.set_byte(0x44, 0);

        cpu.run();

        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_l_to_b() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.memory.set_byte(0x45, 0);

        cpu.run();

        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_hl_to_b() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x46, 0);

        cpu.registers.set_hl(0x2);
        cpu.memory.set_byte(0xFF, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.b, 0xFF);
    }
}
