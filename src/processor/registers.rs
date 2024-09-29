#![allow(dead_code)]

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn set_zero(self: &mut Registers) {
        self.f |= 0b1000000;
    }

    pub fn set_carry(self: &mut Registers) {
        self.f |= 0b1000;
    }

    pub fn set_n(self: &mut Registers) {
        self.f |= 0b100000;
    }

    pub fn set_h(self: &mut Registers) {
        self.f |= 0b10000;
    }

    pub fn af(self: &mut Registers) -> u16 {
        combine(self.a as u16, self.f as u16)
    }

    pub fn bc(self: &mut Registers) -> u16 {
        combine(self.b as u16, self.c as u16)
    }

    pub fn de(self: &mut Registers) -> u16 {
        combine(self.d as u16, self.e as u16)
    }

    pub fn hl(self: &mut Registers) -> u16 {
        combine(self.h as u16, self.l as u16)
    }
}

fn combine(high: u16, low: u16) -> u16 {
    (high << 8) | low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn af_return_combine_a_and_f() {
        let mut registers = Registers::new();
        registers.a = 0x8;
        registers.f = 0x1;

        let af = registers.af();
        assert_eq!(af, 0x801);
    }

    #[test]
    fn bc_return_combine_b_and_c() {
        let mut registers = Registers::new();
        registers.b = 0x8;
        registers.c = 0x0;

        let bc = registers.bc();
        assert_eq!(bc, 0x800);
    }

    #[test]
    fn de_return_combine_d_and_e() {
        let mut registers = Registers::new();
        registers.d = 0x0;
        registers.e = 0x11;

        let de = registers.de();
        assert_eq!(de, 0x0011);
    }

    #[test]
    fn hl_return_combine_h_and_l() {
        let mut registers = Registers::new();
        registers.h = 0x88;
        registers.l = 0x10;

        let hl = registers.hl();
        assert_eq!(hl, 0x8810);
    }

    #[test]
    fn set_zero_flag() {
        let mut registers = Registers::new();
        registers.f = 0b1;
        registers.set_zero();

        assert_eq!(0b1000001, registers.f);
    }

    #[test]
    fn idempotent_if_already_set_carry_flag() {
        let mut registers = Registers::new();
        registers.f = 0b1001;
        registers.set_carry();

        assert_eq!(0b1001, registers.f);
    }
}
