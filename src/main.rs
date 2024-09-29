use crate::config::*;
use std::time::{Duration, Instant};

use crate::processor::cpu::Cpu;

mod config;
mod processor;

fn main() {
    let mut cpu = Cpu::new();

    main_loop(&mut cpu);
}

fn main_loop(cpu: &mut Cpu) {
    loop {
        let now = Instant::now();
        if cpu.run() == 0 {
            break;
        }
        tick(now);
    }
}

fn tick(now: Instant) {
    while now.elapsed() < Duration::from_nanos(CLOCK_DURATION_NS) {}
}
