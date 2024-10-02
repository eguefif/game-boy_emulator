use crate::processor::registers::{set_h, set_n, set_zero};

use super::add::half_overflow_u8;

pub fn sub(target: &mut u8, value: u8, flags: &mut u8) {
    let (res, _) = target.overflowing_sub(value);
    if res == 0 {
        *flags = set_zero(*flags);
    }
    if half_overflow_u8(*target, value) {
        *flags = set_h(*flags);
    }
    *flags = set_n(*flags);
    *target = res;
}
