use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loadc_dispatch(&mut self, target: Target) {
        match target {
            Target::A => self.registers.c = self.registers.a,
            Target::B => self.registers.c = self.registers.b,
            Target::C => {}
            Target::D => self.registers.c = self.registers.d,
            Target::E => self.registers.c = self.registers.e,
            Target::H => self.registers.c = self.registers.h,
            Target::L => self.registers.c = self.registers.l,
            Target::HL => {
                let position = self.registers.hl() as usize;
                self.registers.c = self.memory.fetch_byte(position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_c() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        cpu.memory.set_byte(0x48, 0);

        cpu.run();

        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_a_to_c() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        cpu.memory.set_byte(0x4F, 0);

        cpu.run();

        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_c_to_c() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        cpu.memory.set_byte(0x49, 0);

        cpu.run();

        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_d_to_c() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x4A, 0);

        cpu.run();

        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_e_to_c() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x4B, 0);

        cpu.run();

        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_h_to_c() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x13;
        cpu.memory.set_byte(0x4C, 0);

        cpu.run();

        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_l_to_c() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.memory.set_byte(0x4D, 0);

        cpu.run();

        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_hl_to_c() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x4E, 0);

        cpu.registers.set_hl(0x2);
        cpu.memory.set_byte(0xFF, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.c, 0xFF);
    }
}
