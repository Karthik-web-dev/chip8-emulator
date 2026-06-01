mod core;
mod platform;

use std::time::{Duration, Instant};
use crate::platform::Platform;
use macroquad::prelude::*;

#[macroquad::main("CHIP-8")]
async fn main() {
    const TIME_STEP: f64 = 1.0 / 60.0;

    let timer_interval = Duration::from_secs_f64(TIME_STEP);
    let mut next_timer_tick = Instant::now();

    //Initialize chip8 instance.
    let mut cpu: crate::core::cpu::Chip8 = crate::core::cpu::Chip8::new();


    // Main loop
    loop {
        //Implementing the timers: 
        cpu.tick();
        Platform::render(&cpu.display);

        while Instant::now() >= next_timer_tick {
            if cpu.dt > 0 {
                cpu.dt -= 1;
            }
            if cpu.st > 0 {
                cpu.st -= 1;
            }
            next_timer_tick += timer_interval;
        }

        next_frame().await;
    }
}

