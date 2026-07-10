use core::{
    hint::unreachable_unchecked,
    mem::{MaybeUninit, transmute},
};

use ch32_hal::{
    Peri,
    peripherals::{self, DMA1_CH3, PA7, SPI1},
    spi::{BitOrder, Config, Spi},
    time::Hertz,
};
use embassy_time::Timer;

use crate::{COLOR_COUNT, color::Color};

/// The SPI frequency, chosen based on the timing requirements of the LEDs on the board.
///
/// WS2812 LEDs communicate by sending either a short high and long low pulse (0 code) or a long
/// high and short low pulse (1 code). The timing depends on the model, but WS2812B-2020 LEDs
/// require 220ns-380ns for a short pulse and 580ns-1000ns for a long pulse.
///
/// For our purposes, it is most efficient to choose a long pulse duration that is 2x (for memory
/// usage) or 3x (for speed/simplicity) the duration of the short pulse, as this means we need less
/// SPI transfers/clocks to send the same pulses. This narrows the duration range for the short
/// pulse down to 290ns-380ns and 220ns-333ns respectively.
///
/// Additionally, on the CH32V203, the SPI frequency must be an integer division of the system
/// clock, which is configured as 144MHz (the maximum available). We also prefer higher frequencies
/// as this means transfers take less time. A division of 32 (4.5MHz) results in a clock interval of
/// 222.22ns, with a long pulse of 666.67ns at 3x that duration.
const SPI_FREQ: u32 = 4_500_000;

pub struct Ws2812b {
    spi: Spi<'static, peripherals::SPI1, ch32_hal::mode::Async>,
}

impl Ws2812b {
    pub async fn new(
        peri: Peri<'static, SPI1>,
        mosi: Peri<'static, PA7>,
        dma: Peri<'static, DMA1_CH3>,
    ) -> Self {
        let mut config = Config::default();
        config.bit_order = BitOrder::MsbFirst;
        config.frequency = Hertz::hz(SPI_FREQ);

        // Don't actually use SPI. We just use the peripheral to bitbang the protocol, meaning we
        // only need 1 pin to be configured.
        let mut spi = Spi::new_txonly_nosck(peri, mosi, dma, config);

        // ensure TX pin is not floating and ensure LED is not trying to read the bogus data
        _ = spi.write(&[0_u8]).await;
        Timer::after_micros(280).await;

        Self { spi }
    }

    pub async fn set_colors(&mut self, colors: &[Color; COLOR_COUNT]) -> embassy_time::Timer {
        // Each color requires 24 codes (8 per channel) and each code requires 4 transfers, meaning
        // 12 bytes per color. For an 8x8 matrix, this means a 768 byte DMA buffer.
        // TODO: this probably does not belong on the stack
        let mut dma_buffer = [const { MaybeUninit::<u8>::uninit() }; COLOR_COUNT * 3 * 4];

        for (color_i, color) in colors.iter().enumerate() {
            let color_start = color_i * 3 * 4;

            for (component_i, component) in color.get_components().into_iter().enumerate() {
                let component_start = color_start + component_i * 4;

                #[allow(clippy::identity_op)]
                dma_buffer[component_start + 0].write(Self::get_double_bit_pattern(component, 6));
                dma_buffer[component_start + 1].write(Self::get_double_bit_pattern(component, 4));
                dma_buffer[component_start + 2].write(Self::get_double_bit_pattern(component, 2));
                dma_buffer[component_start + 3].write(Self::get_double_bit_pattern(component, 0));
            }
        }

        // SAFETY: just initialized each byte
        // TODO: use MaybeUninit::transpose once it stabilizes, see rust-lang/rust#96097
        let dma_buffer: [u8; COLOR_COUNT * 3 * 4] = unsafe { transmute(dma_buffer) };

        // NOTE: implementation returns no errors
        _ = self.spi.write(&dma_buffer).await;

        // return delay that resolves after
        embassy_time::Timer::after_micros(280)
    }

    const fn get_double_bit_pattern(num: u8, idx: usize) -> u8 {
        match (num >> idx) & 0b11 {
            0b00 => 0b1000_1000,
            0b01 => 0b1000_1110,
            0b10 => 0b1110_1000,
            0b11 => 0b1110_1110,
            // SAFETY: bitwise mask verifies this
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
