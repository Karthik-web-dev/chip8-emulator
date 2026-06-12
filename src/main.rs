mod core;
mod platform;

use std::env;
use std::time::{Duration, Instant};
use crate::platform::Platform;
use macroquad::prelude::*;

#[macroquad::main("CHIP-8")]
async fn main() {
    println!("Running");
    //let args: Vec<String> = env::args().collect();

    const TIME_STEP: f64 = 1.0 / 60.0;
    const TICKS_PER_FRAME: u32 = 20;

    let timer_interval = Duration::from_secs_f64(TIME_STEP);
    let mut next_timer_tick = Instant::now();

    //Initialize chip8 instance.
    let mut cpu: crate::core::cpu::Chip8 = crate::core::cpu::Chip8::new();
    
    match env::args().nth(1) {
        Some(rom_path) => {
            println!("Loading ROM: {}", rom_path);
            cpu.memory.load_rom(&rom_path);
        }
        None => {
            eprintln!("Usage: cargo run <rom>");
            std::process::exit(1);
        }
    }


    // Main loop
    loop {
        //Implementing the timers:
        Platform::check_keyboard(&mut cpu.keypad);
        for _ in 0..TICKS_PER_FRAME {
            cpu.tick();
        }
        Platform::render(&cpu.display);

        while Instant::now() >= next_timer_tick {
            if cpu.dt > 0 {
                cpu.dt -= 1;
            }
            if cpu.st > 0 {
                cpu.st -= 1;
                println!("\x07");
                //audio.play();
            } else {
                //audio.stop();
            }
            next_timer_tick += timer_interval;
        }

        next_frame().await;
    }
}

