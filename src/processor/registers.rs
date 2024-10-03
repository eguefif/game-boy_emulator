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
    pub pc: usize,
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
            pc: 0,
        }
    }

    pub fn set_af(self: &mut Registers, value: u16) {
        (self.a, self.f) = get_high_and_low(value);
    }

    pub fn set_bc(self: &mut Registers, value: u16) {
        (self.b, self.c) = get_high_and_low(value);
    }

    pub fn set_de(self: &mut Registers, value: u16) {
        (self.d, self.e) = get_high_and_low(value);
    }

    pub fn set_hl(self: &mut Registers, value: u16) {
        (self.h, self.l) = get_high_and_low(value);
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

    pub fn set_zero(self: &mut Registers) {
        self.f = self.f | 0b1000_0000
    }

    pub fn unset_zero(self: &mut Registers) {
        self.f = self.f & 0b0111_1111;
    }

    pub fn set_carry(self: &mut Registers) {
        self.f = self.f | 0b0001_0000;
    }

    pub fn unset_carry(self: &mut Registers) {
        self.f = self.f & 0b1110_1111;
    }

    pub fn set_n(self: &mut Registers) {
        self.f = self.f | 0b0100_0000;
    }

    pub fn unset_n(self: &mut Registers) {
        self.f = self.f & 0b1011_1111;
    }

    pub fn set_h(self: &mut Registers) {
        self.f = self.f | 0b0010_0000;
    }

    pub fn unset_h(self: &mut Registers) {
        self.f = self.f & 0b1101_1111;
    }

    pub fn get_carry(self: &mut Registers) -> bool {
        (self.f & 0b0001_0000) == 0b0001_0000
    }

    pub fn get_zero(self: &mut Registers) -> bool {
        (self.f & 0b1000_0000) == 0b1000_0000
    }

    pub fn get_half_carry(self: &mut Registers) -> bool {
        (self.f & 0b0010_0000) == 0b0010_0000
    }

    pub fn get_n(self: &mut Registers) -> bool {
        (self.f & 0b0100_0000) == 0b0100_0000
    }
}

fn combine(high: u16, low: u16) -> u16 {
    (high << 8) | low
}

fn get_high_and_low(value: u16) -> (u8, u8) {
    let high = ((value >> 8) & 0xFF) as u8;
    let low = (value & 0xFF) as u8;

    (high, low)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bc_should_set_b_and_c() {
        let mut registers = Registers::new();
        registers.set_bc(0x3f0);

        assert_eq!(registers.b, 0x3);
        assert_eq!(registers.c, 0xf0);
    }

    #[test]
    fn set_af_should_set_a_and_f() {
        let mut registers = Registers::new();
        registers.set_af(0xF301);

        assert_eq!(registers.a, 0xF3);
        assert_eq!(registers.f, 0x1);
    }

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
        registers.set_zero();

        assert_eq!(0b1000_0000, registers.f);
    }

    #[test]
    fn unset_zero_flag() {
        let mut registers = Registers::new();
        registers.set_zero();
        registers.unset_zero();

        assert_eq!(0b1000_0000, registers.f);
    }

    #[test]
    fn idempotent_if_already_set_carry_flag() {
        let mut registers = Registers::new();
        registers.set_carry();

        assert_eq!(0b0001_0001, registers.f);
    }
}
