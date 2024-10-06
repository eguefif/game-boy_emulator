use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loadd_dispatch(&mut self, target: Target) {
        match target {
            Target::A => self.registers.d = self.registers.a,
            Target::B => self.registers.d = self.registers.b,
            Target::C => self.registers.d = self.registers.c,
            Target::D => {}
            Target::E => self.registers.d = self.registers.e,
            Target::H => self.registers.d = self.registers.h,
            Target::L => self.registers.d = self.registers.l,
            Target::HL => {
                let position = self.registers.hl() as usize;
                self.registers.d = self.memory.fetch_byte(position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_d() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        cpu.memory.set_byte(0x50, 0);

        cpu.run();

        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_a_to_d() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        cpu.memory.set_byte(0x57, 0);

        cpu.run();

        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_c_to_d() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        cpu.memory.set_byte(0x51, 0);

        cpu.run();

        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_d_to_d() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x52, 0);

        cpu.run();

        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_e_to_d() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x53, 0);

        cpu.run();

        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_h_to_d() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x13;
        cpu.memory.set_byte(0x54, 0);

        cpu.run();

        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_l_to_d() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.memory.set_byte(0x55, 0);

        cpu.run();

        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_hl_to_d() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x56, 0);

        cpu.registers.set_hl(0x2);
        cpu.memory.set_byte(0xFF, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.d, 0xFF);
    }
}
