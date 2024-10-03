use crate::processor::{
    cpu::Cpu,
    registers::{get_carry, set_carry, set_h, set_n_to_zero, set_zero},
};

// TODO: move add function in there files with tests
// TODO: move check overflow in there files with tests
// TODO: test only opcode behavior and flow here
// TODO: refactor add functions ?

impl Cpu {
    pub fn add_dispatch(self: &mut Cpu, opcode: u8) {
        let low_opcode = opcode & 0xf;
        match low_opcode {
            0 => add(
                &mut self.registers.a,
                self.registers.b,
                &mut self.registers.f,
            ),
            1 => add(
                &mut self.registers.a,
                self.registers.c,
                &mut self.registers.f,
            ),
            2 => add(
                &mut self.registers.a,
                self.registers.d,
                &mut self.registers.f,
            ),
            3 => add(
                &mut self.registers.a,
                self.registers.e,
                &mut self.registers.f,
            ),
            4 => add(
                &mut self.registers.a,
                self.registers.h,
                &mut self.registers.f,
            ),
            5 => add(
                &mut self.registers.a,
                self.registers.l,
                &mut self.registers.f,
            ),
            6 => {
                let address = self.registers.hl() as usize;
                let value = self.memory[address];
                add(&mut self.registers.a, value, &mut self.registers.f);
            }
            7 => {
                let a_value = self.registers.a;
                add(&mut self.registers.a, a_value, &mut self.registers.f);
            }
            8 => add_with_carry(
                &mut self.registers.a,
                self.registers.b,
                &mut self.registers.f,
            ),
            0x9 => add_with_carry(
                &mut self.registers.a,
                self.registers.c,
                &mut self.registers.f,
            ),
            0xa => add_with_carry(
                &mut self.registers.a,
                self.registers.d,
                &mut self.registers.f,
            ),
            0xb => add_with_carry(
                &mut self.registers.a,
                self.registers.e,
                &mut self.registers.f,
            ),
            0xc => add_with_carry(
                &mut self.registers.a,
                self.registers.h,
                &mut self.registers.f,
            ),
            0xd => add_with_carry(
                &mut self.registers.a,
                self.registers.l,
                &mut self.registers.f,
            ),
            0xe => {
                let address = self.registers.hl() as usize;
                let value = self.memory[address];
                add_with_carry(&mut self.registers.a, value, &mut self.registers.f);
            }
            0xf => {
                let a_value = self.registers.a;
                add_with_carry(&mut self.registers.a, a_value, &mut self.registers.f);
            }
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
    }
}
pub fn add(target: &mut u8, value: u8, flags: &mut u8) {
    let (res, overflow) = target.overflowing_add(value);
    if res == 0 {
        *flags = set_zero(*flags);
    }
    if overflow {
        *flags = set_carry(*flags);
    }
    if half_overflow_u8(*target, value) {
        *flags = set_h(*flags);
    }
    *flags = set_n_to_zero(*flags);
    *target = res;
}

pub fn add_with_carry(target: &mut u8, value: u8, flags: &mut u8) {
    let carry = if get_carry(*flags) { 1 } else { 0 };
    let (tmp, overflow) = target.overflowing_add(value);
    let (res, overflow2) = tmp.overflowing_add(carry);
    if res == 0 {
        *flags = set_zero(*flags);
    }
    if overflow || overflow2 {
        *flags = set_carry(*flags);
    }
    if half_overflow_u8(*target, value) {
        *flags = set_h(*flags);
    }
    *target = res;
    *flags = set_n_to_zero(*flags);
}

pub fn half_overflow_u8(target: u8, value: u8) -> bool {
    let low_target_nibble = target & 0xF;
    let low_value_nibble = value & 0xF;
    ((low_target_nibble + low_value_nibble) & 0x10) == 0x10
}

pub fn half_overflow_u16(target: u16, value: u16) -> bool {
    let low_target_nibble = target & 0xFFF;
    let low_value_nibble = value & 0xFFF;
    ((low_target_nibble + low_value_nibble) & 0x1000) == 0x1000
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_check_half_overflow_u8_true() {
        let a = 0xf;
        let b = 0x1;

        let res = half_overflow_u8(a, b);
        assert!(res);
    }

    #[test]
    fn it_check_half_overflow_u8_false() {
        let a = 0xe;
        let b = 0x1;

        let res = half_overflow_u8(a, b);
        assert!(!res);
    }

    #[test]
    fn it_check_half_overflow_u16_true() {
        let a = 0xfff;
        let b = 0x1;

        let res = half_overflow_u16(a, b);
        assert!(res);
    }

    #[test]
    fn it_check_half_overflow_u16_false() {
        let a = 0xffe;
        let b = 0x1;

        let res = half_overflow_u16(a, b);
        assert!(!res);
    }

    #[test]
    fn it_add_with_no_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x1;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x1 + 0x1);
        assert_eq!(cpu.registers.f, 0b0000000);
    }

    //test add regular add
    #[test]
    fn it_add_with_overflow_and_set_carry_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x80);

        println!("{:b}", cpu.registers.a);
        assert_eq!(cpu.registers.a, 0x0);
        assert_eq!(cpu.registers.f, 0b1011_0000);
    }
    #[test]
    fn it_add_with_overflow_and_set_carry_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.b = 0x2;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x1);
        assert_eq!(cpu.registers.f, 0b0011_0000);
    }

    #[test]
    fn it_add_with_low_overflow_and_set_half_carry_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xF;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x1 + 0xF);
        assert_eq!(cpu.registers.f, 0b010_0000);
    }

    #[test]
    fn it_add_nothing_and_set_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x0;
        cpu.registers.b = 0x0;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x0);
        assert_eq!(cpu.registers.f, 0b1000_0000);
    }

    #[test]
    fn it_add_memory_content_at_hl_to_ad_no_flag() {
        let mut cpu = Cpu::new();
        let address: usize = 0xff;
        cpu.memory[address] = 0x1;
        cpu.registers.a = 0x2;
        cpu.registers.set_hl(address as u16);
        cpu.add_dispatch(0x86);

        assert_eq!(cpu.registers.a, 0x1 + 0x2);
        assert_eq!(cpu.registers.f, 0b000000);
    }

    #[test]
    fn it_add_from_register_c() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.c = 0x2;
        cpu.add_dispatch(0x81);

        assert_eq!(cpu.registers.a, 0x6);
        assert_eq!(cpu.registers.f, 0b0000000);
    }

    #[test]
    fn it_add_from_register_d() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.d = 0x2;
        cpu.add_dispatch(0x82);

        assert_eq!(cpu.registers.a, 0x6);
        assert_eq!(cpu.registers.f, 0b0000000);
    }

    #[test]
    fn it_add_from_register_e() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.e = 0x2;
        cpu.add_dispatch(0x83);

        assert_eq!(cpu.registers.a, 0x6);
        assert_eq!(cpu.registers.f, 0b0000000);
    }

    #[test]
    fn it_add_from_register_h() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.h = 0x2;
        cpu.add_dispatch(0x84);

        assert_eq!(cpu.registers.a, 0x6);
        assert_eq!(cpu.registers.f, 0b0000000);
    }

    #[test]
    fn it_add_from_register_l() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.l = 0x2;
        cpu.add_dispatch(0x85);

        assert_eq!(cpu.registers.a, 0x6);
        assert_eq!(cpu.registers.f, 0b0000000);
    }

    //*************************** test add with carry
    #[test]
    fn it_add_with_carry_with_no_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.registers.a = 0x1;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x88);

        assert_eq!(cpu.registers.a, 0x1 + 0x1 + 0x1);
        assert_eq!(cpu.registers.f, 0b0001_0000);
    }

    #[test]
    fn it_add_with_carry_with_overflow_and_set_carry_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.registers.a = 0xFF;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x88);

        assert_eq!(cpu.registers.a, 0x1);
        assert_eq!(cpu.registers.f, 0b011_0000);
    }

    #[test]
    fn it_add_with_carry_with_low_overflow_and_set_half_carry_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.registers.a = 0xe;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x88);

        assert_eq!(cpu.registers.a, 0xe + 0x1 + 0x1);
        assert_eq!(cpu.registers.f, 0b001_0000);
    }

    #[test]
    fn it_add_with_carry_memory_content_at_hl_to_ad_no_flag() {
        let mut cpu = Cpu::new();
        let address: usize = 0xff;
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.memory[address] = 0x3;
        cpu.registers.a = 0x2;
        cpu.registers.set_hl(address as u16);
        cpu.add_dispatch(0x8e);

        assert_eq!(cpu.registers.a, 0x3 + 0x2 + 0x1);
        assert_eq!(cpu.registers.f, 0b001_0000);
    }

    #[test]
    fn it_add_with_carry_from_register_c() {
        let mut cpu = Cpu::new();
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.registers.a = 0x4;
        cpu.registers.c = 0x2;
        cpu.add_dispatch(0x89);

        assert_eq!(cpu.registers.a, 0x7);
        assert_eq!(cpu.registers.f, 0b001_0000);
    }

    #[test]
    fn it_add_with_carry_from_register_d() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.registers.d = 0x2;
        cpu.add_dispatch(0x8a);

        assert_eq!(cpu.registers.a, 0x7);
        assert_eq!(cpu.registers.f, 0b001_0000);
    }

    #[test]
    fn it_add_with_carry_from_register_e() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.e = 0x2;
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.add_dispatch(0x8b);

        assert_eq!(cpu.registers.a, 0x7);
        assert_eq!(cpu.registers.f, 0b001_0000);
    }

    #[test]
    fn it_add_with_carry_from_register_h() {
        let mut cpu = Cpu::new();
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.registers.a = 0x4;
        cpu.registers.h = 0x2;
        cpu.add_dispatch(0x8c);

        assert_eq!(cpu.registers.a, 0x7);
        assert_eq!(cpu.registers.f, 0b001_0000);
    }

    #[test]
    fn it_add_with_carry_from_register_l() {
        let mut cpu = Cpu::new();
        cpu.registers.f = set_carry(cpu.registers.f);
        cpu.registers.a = 0x4;
        cpu.registers.l = 0x2;
        cpu.add_dispatch(0x8d);

        assert_eq!(cpu.registers.a, 0x7);
        assert_eq!(cpu.registers.f, 0b001_0000);
    }
}
