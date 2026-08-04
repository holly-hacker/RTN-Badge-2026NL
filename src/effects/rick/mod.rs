use core::mem::transmute;

use embassy_time::Duration;

use crate::{PIXEL_COUNT, color::Color};

/// Video data, generated like so:
/// ```sh
/// ffmpeg -y -i /path/to/rick.mp4 -vf "crop=ih:ih,fps=8,eq=saturation=1.6:contrast=1.3,scale=8:8:flags=area" \
/// -sws_dither none -f rawvideo -pix_fmt rgb565le -t 6 -an rick.dat
/// ```
///
/// This is currently about 6kb of uncompressed video data.
///
/// This is temporarily wrapped into an AlignedVideoData to align it to 2 bytes, then transmuted
/// into an array of u16.
static VIDEO_DATA: &[u16; AlignedVideoData::PIXELS] =
    &AlignedVideoData::new(*include_bytes!("rick.dat")).into_u16();

const BYTES_PER_PIXEL: usize = 2; // RGB565
const DURATION: usize = 6; // in seconds
const FRAME_RATE: usize = 8; // in frames per second

const FRAME_COUNT: usize = FRAME_RATE * DURATION;
const BYTES_PER_FRAME: usize = BYTES_PER_PIXEL * PIXEL_COUNT;

pub fn rick(time: Duration, buf: &mut [Color; PIXEL_COUNT]) {
    const MS_PER_FRAME: usize = 1000 / FRAME_RATE;

    let elapsed_ms = time.as_millis();
    let absolute_frame = elapsed_ms / MS_PER_FRAME as u64;
    let frame_index = (absolute_frame % FRAME_COUNT as u64) as usize;

    // NOTE: compiler should know that this slice has length `PIXEL_COUNT`
    let frame_slice = &VIDEO_DATA[frame_index * PIXEL_COUNT..][..PIXEL_COUNT];

    for (i, color) in frame_slice.iter().enumerate() {
        // extract 5bit colors
        let (r, g, b) = (
            (color >> 11) & 0b11111,
            (color >> 5) & 0b111111,
            color & 0b11111,
        );

        // shift colors to most significant bits
        let (r, g, b) = ((r << 3) as u8, (g << 2) as u8, (b << 3) as u8);

        buf[i] = Color::new(r, g, b);
    }
}

#[repr(C)]
pub struct AlignedVideoData {
    /// Aligned to 2 bytes by default
    pub _align: [u16; 0],
    /// Aligned to 2 bytes because it is laid out after `_align` thanks to `repr(C)`
    pub bytes: [u8; Self::SIZE_BYTES],
}

impl AlignedVideoData {
    const SIZE_BYTES: usize = BYTES_PER_FRAME * FRAME_COUNT;
    const PIXELS: usize = PIXEL_COUNT * FRAME_COUNT;

    pub const fn new(bytes: [u8; Self::SIZE_BYTES]) -> Self {
        Self { _align: [], bytes }
    }

    pub const fn into_u16(self) -> [u16; Self::PIXELS] {
        // SAFETY: trust me bro
        unsafe { transmute(self.bytes) }
    }
}
