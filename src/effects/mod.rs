use embassy_time::Duration;

use crate::{COLOR_COUNT, color::Color};

pub type Effect = fn(time: Duration, frame: usize, buf: &mut [Color; COLOR_COUNT]);

pub const ALL_EFFECTS: [Effect; 2] = [pixel_run, pixel_counter];

fn pixel_run(time: Duration, _frame: usize, buf: &mut [Color; COLOR_COUNT]) {
    let index = (time.as_millis() / 64) as usize % COLOR_COUNT;
    for (i, item) in buf.iter_mut().enumerate() {
        *item = Color::from_bool(index == i);
    }
}

fn pixel_counter(time: Duration, _frame: usize, buf: &mut [Color; COLOR_COUNT]) {
    let elapsed_ms = time.as_millis();

    for (i, item) in buf.iter_mut().enumerate() {
        let is_set = (elapsed_ms & 1 << i) > 0;
        *item = Color::from_bool_dim(is_set);
    }
}
