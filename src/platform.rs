use macroquad::prelude::*;
pub struct Platform;

const SCALE: f32 = 30.0;

impl Platform {
    pub fn render(display: &crate::core::cpu::Display) {
        for (i, pixel) in display.pixels.iter().enumerate() {
            let x_cord: f32 = (i % 64) as f32;
            let y_cord: f32 = (i / 64) as f32;

            if *pixel == 1 {
                draw_rectangle(x_cord * SCALE, y_cord * SCALE, SCALE, SCALE, WHITE)
            }
        }
        draw_grid();
    }

pub fn check_keyboard(keypad: &mut [u8; 16]) {
    let mapping = [
        (KeyCode::X,     0x0usize),
        (KeyCode::Key1,  0x1),
        (KeyCode::Key2,  0x2),
        (KeyCode::Key3,  0x3),
        (KeyCode::Q,     0x4),
        (KeyCode::W,     0x5),
        (KeyCode::E,     0x6),
        (KeyCode::A,     0x7),
        (KeyCode::S,     0x8),
        (KeyCode::D,     0x9),
        (KeyCode::Z,     0xA),
        (KeyCode::C,     0xB),
        (KeyCode::Key4,  0xC),
        (KeyCode::R,     0xD),
        (KeyCode::F,     0xE),
        (KeyCode::V,     0xF),
    ];

    for (key, chip8_key) in mapping.iter() {
        keypad[*chip8_key] = if is_key_down(*key) { 1 } else { 0 };
    }
}
}

pub fn draw_grid() {
    let width = 64;
    let height = 32;

    let screen_w = width as f32 * SCALE;
    let screen_h = height as f32 * SCALE;

    // vertical lines
    for x in 0..=width {
        let x_pos = x as f32 * SCALE;
        draw_line(x_pos, 0.0, x_pos, screen_h, 0.2, GRAY);
    }

    // horizontal lines
    for y in 0..=height {
        let y_pos = y as f32 * SCALE;
        draw_line(0.0, y_pos, screen_w, y_pos, 0.2, GRAY);
    }

    // optional border (clean frame)
    draw_rectangle_lines(0.0, 0.0, screen_w, screen_h, 2.0, WHITE);
}
