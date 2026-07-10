use embassy_time::Duration;

use crate::{COLOR_COUNT, color::Color};

pub type Effect = fn(time: Duration, frame: usize, buf: &mut [Color; COLOR_COUNT]);

pub const ALL_EFFECTS: [Effect; 2] = [pixel_run, pixel_counter];

fn pixel_run(_time: Duration, frame: usize, buf: &mut [Color; COLOR_COUNT]) {
    let index = frame % COLOR_COUNT;
    for (i, item) in buf.iter_mut().enumerate() {
        *item = (index == i).into();
    }
}

fn pixel_counter(time: Duration, _frame: usize, buf: &mut [Color; COLOR_COUNT]) {
    let elapsed_ms = time.as_ticks();

    for (i, item) in buf.iter_mut().enumerate() {
        let is_set = (elapsed_ms & 1 << i) > 0;
        *item = is_set.into();
    }
}
