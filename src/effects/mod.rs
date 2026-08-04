mod rick;
mod rtn_logo;

use embassy_time::Duration;
use fixed::types::{I16F16, I48F16};
use fixed_macro::types::I16F16;

use crate::{PIXEL_COUNT, color::Color, utils::math::fast_cos};

pub type Effect = fn(time: Duration, buf: &mut [Color; PIXEL_COUNT]);

pub const ALL_EFFECTS: [Effect; 5] = [
    (rtn_logo::rtn_logo),
    gradient,
    pixel_run,
    pixel_counter,
    (rick::rick),
];

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

fn set_colors_to_bitmask(buf: &mut [Color; PIXEL_COUNT], bitmask: u64) {
    for (i, item) in buf.iter_mut().enumerate() {
        let is_set = (bitmask & 1 << i) > 0;
        *item = Color::from_bool(is_set);
    }
}
