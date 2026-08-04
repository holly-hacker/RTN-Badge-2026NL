use embassy_time::Duration;
use fixed::types::{I16F16, I48F16};
use fixed_macro::types::I16F16;

use crate::{PIXEL_COUNT, color::Color, utils::math::fast_cos};

pub type Effect = fn(time: Duration, buf: &mut [Color; PIXEL_COUNT]);

pub const ALL_EFFECTS: [Effect; 4] = [rtn_logo, gradient, pixel_run, pixel_counter];

fn pixel_run(time: Duration, buf: &mut [Color; PIXEL_COUNT]) {
    let index = (time.as_millis() / 64) as usize % PIXEL_COUNT;
    let mask = 1 << index;
    set_colors_to_bitmask(buf, mask);
}

fn pixel_counter(time: Duration, buf: &mut [Color; PIXEL_COUNT]) {
    let elapsed_ms = time.as_millis();
    set_colors_to_bitmask(buf, elapsed_ms);
}

/// A scrolling gradient. Based on the default shader on shadertoy.
fn gradient(time: Duration, buf: &mut [Color; PIXEL_COUNT]) {
    let time = time.as_millis() as u32;
    let time = time * 2; // double speed
    let time = I48F16::from_num(time) / 1000; // NOTE: no 64bit hardware
    let time = I16F16::from_num(time);
    let time = time % (I16F16::PI * 2);

    for (i, item) in buf.iter_mut().enumerate() {
        let x = I16F16::from_num(i % 8) / 4;
        let y = I16F16::from_num(i / 8) / 4;

        // vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));
        let r = I16F16!(0.5) + I16F16!(0.5) * fast_cos(time + x);
        let g = I16F16!(0.5) + I16F16!(0.5) * fast_cos(time + y + I16F16!(2));
        let b = I16F16!(0.5) + I16F16!(0.5) * fast_cos(time + x + I16F16!(4));

        let r: u8 = (r * 255).to_num();
        let g: u8 = (g * 255).to_num();
        let b: u8 = (b * 255).to_num();

        *item = Color::new(r, g, b);
    }
}

/// An animation showing the letters `R`, `T` and `N` in sequence
fn rtn_logo(time: Duration, buf: &mut [Color; PIXEL_COUNT]) {
    const LETTER_R: u64 = 0b11111100_11111110_11001110_11001110_11111100_11111100_11001110_11000110;
    const LETTER_T: u64 = 0b11111111_11111111_00011000_00011000_00011000_00011000_00011000_00011000;
    const LETTER_N: u64 = 0b11110011_11110011_11111011_11111011_11011111_11001111_11001111_11000111;
    const MASKS: [u64; 3] = [
        LETTER_R.reverse_bits(),
        LETTER_T.reverse_bits(),
        LETTER_N.reverse_bits(),
    ];

    // animation sequence
    const ANIMATION_FADE: u32 = 500;
    const ANIMATION_SOLID: u32 = 1500;
    const ANIMATION_SLIDE: u32 = 2000;
    const ANIMATION_END: u32 = 2500;

    let letter_idx = (time.as_millis() / ANIMATION_END as u64) as usize % MASKS.len();
    let letter = MASKS[letter_idx];
    let phase = (time.as_millis() % ANIMATION_END as u64) as u32;

    let (brightness, shift): (u8, (i32, i32)) = if phase < ANIMATION_FADE {
        ((phase * 255 / ANIMATION_FADE) as u8, (0, 0))
    } else if phase < ANIMATION_SOLID {
        (255, (0, 0))
    } else if phase < ANIMATION_SLIDE {
        let time_in_phase = phase - ANIMATION_SOLID;
        let slide_phase = time_in_phase * 8 / (ANIMATION_SLIDE - ANIMATION_SOLID);
        let slide_x = match letter_idx {
            0 => -(slide_phase as i32),
            1 => 0,
            _ => slide_phase as i32,
        };
        let slide_y = match letter_idx {
            1 => slide_phase as i32,
            _ => 0,
        };
        (255, (slide_x, slide_y))
    } else {
        (0, (0, 0))
    };

    // shift y
    let letter = shl_signed_u64(letter, shift.1 * 8);

    // shift x
    let letter = shl_signed_u64(letter, shift.0);
    // unset bits that crossed byte boundary
    let byte_mask = shl_signed_u8(0xFFu8, shift.0);
    let letter = letter & splat_u8_to_u64(byte_mask);

    for (i, item) in buf.iter_mut().enumerate() {
        *item = if letter & 1 << i > 0 {
            Color::new_w(brightness)
        } else {
            Color::OFF
        };
    }
}

fn set_colors_to_bitmask(buf: &mut [Color; PIXEL_COUNT], bitmask: u64) {
    for (i, item) in buf.iter_mut().enumerate() {
        let is_set = (bitmask & 1 << i) > 0;
        *item = Color::from_bool(is_set);
    }
}

#[allow(clippy::erasing_op, clippy::identity_op)]
const fn splat_u8_to_u64(byte: u8) -> u64 {
    let b = byte as u64;

    b << (8 * 0)
        | b << (8 * 1)
        | b << (8 * 2)
        | b << (8 * 3)
        | b << (8 * 4)
        | b << (8 * 5)
        | b << (8 * 6)
        | b << (8 * 7)
}

const fn shl_signed_u8(value: u8, amount: i32) -> u8 {
    if amount >= 0 {
        value << amount
    } else {
        value >> -amount
    }
}

const fn shl_signed_u64(value: u64, amount: i32) -> u64 {
    if amount >= 0 {
        value << amount
    } else {
        value >> -amount
    }
}
