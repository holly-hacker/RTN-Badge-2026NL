use embassy_time::Duration;

use crate::{PIXEL_COUNT, PIXEL_WIDTH, color::Color};

const TEXT_LEN: usize = 22 + 2;
const TEXT: [u32; TEXT_LEN] = {
    use font::*;
    [
        G, r, e, e, t, i, n, g, s, /**/ f, r, o, m, /**/ V, a, r, i, a, n, t, NINE,
        EXCLAIM, PAD, PAD,
    ]
};
const SPACES: [usize; 2] = [9, 13];

const CHAR_WIDTH: usize = 4;
const CHAR_SPACING: usize = 1;
const SPACE_WIDTH: usize = 3;

const CHAR_INTERVAL: usize = CHAR_WIDTH + CHAR_SPACING;

pub fn greetings(time: Duration, buf: &mut [Color; PIXEL_COUNT]) {
    // reset grid
    buf.iter_mut().for_each(|c| *c = Color::OFF);

    let frame_abs = time.as_millis() / 128;
    let frame = (frame_abs % (TEXT_LEN * CHAR_INTERVAL) as u64) as isize - PIXEL_WIDTH as isize;

    for (i, letter) in TEXT.iter().enumerate() {
        let space_offset = SPACES.into_iter().filter(|&idx| idx <= i).count() * SPACE_WIDTH;

        // span of the letter in the coordinate space of the full message
        let range_start = i * CHAR_INTERVAL + space_offset;
        let range_end = range_start + CHAR_INTERVAL + space_offset; // exclusive

        // span of the letter on the screen, taking into account scrolling
        let scroll_start = range_start as isize - frame;
        let scroll_end = range_end as isize - frame;

        // skip if letter is not in frame
        if scroll_start >= (PIXEL_WIDTH as isize) || scroll_end < 0 {
            // nothing is in range
            continue;
        }

        for bit_x in 0..CHAR_WIDTH {
            let grid_x = scroll_start + (3 - bit_x) as isize;

            if grid_x < 0 || grid_x >= PIXEL_WIDTH as isize {
                continue; // column not in frame
            }

            for bit_y in 0..8 {
                // TODO: horizontal offset

                let grid_y = 7 - bit_y as isize;
                let bit = (letter >> (bit_y * 4 + bit_x) & 1) > 0;

                if bit {
                    let buf_idx = grid_y as usize * 8 + (grid_x as usize);

                    // TODO: colors
                    buf[buf_idx] = Color::WHITE;
                }
            }
        }
    }
}

#[allow(non_upper_case_globals)]
mod font {
    //! Particular letters from the Creep font, manually transcribed into bitmaps.
    //!
    //! Source: https://github.com/romeovs/creep/blob/master/screens/screen.png
    //!
    //! The Creep font is licensed under the MIT license.

    pub const PAD: u32 = 0;
    pub const EXCLAIM: u32 = 0b_0100_0100_0100_0100_0000_0100_0000; // shifted left by 1
    pub const NINE: u32 = 0b_0110_1001_1001_0111_0001_1001_0110;

    pub const a: u32 = 0b_0000_0111_1001_1001_1001_0111_0000;
    pub const e: u32 = 0b_0000_0110_1001_1111_1000_0111_0000;
    pub const f: u32 = 0b_0010_0101_0100_1110_0100_0100_1000;
    pub const g: u32 = 0b_0000_0111_1001_1001_0111_0001_0110;
    pub const i: u32 = 0b_0100_0000_1100_0100_0100_0110_0000;
    pub const m: u32 = 0b_0000_1001_1111_1001_1001_1001_0000;
    pub const n: u32 = 0b_0000_1110_1001_1001_1001_1001_0000;
    pub const o: u32 = 0b_0000_0110_1001_1001_1001_0110_0000;
    pub const r: u32 = 0b_0000_1110_1001_1000_1000_1000_0000;
    pub const s: u32 = 0b_0000_0111_1000_0110_0001_1110_0000;
    pub const t: u32 = 0b_0100_0100_1110_0100_0100_0010_0000;

    pub const G: u32 = 0b_0110_1000_1000_1011_1001_0110_0000;
    pub const V: u32 = 0b_1001_1001_1001_1001_1001_0110_0000;
}
