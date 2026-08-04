#![no_std]
#![no_main]

mod color;
mod effects;
mod hardware;
mod utils;

use ch32_hal::Peri;
use ch32_hal::gpio::{AnyPin, Level, Output};
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use panic_halt as _;

use crate::color::Color;
use crate::hardware::{Button, Ws2812b};

pub const PIXEL_WIDTH: usize = 8;
pub const PIXEL_HEIGHT: usize = 8;

/// The amount of LEDs on the board
pub const PIXEL_COUNT: usize = PIXEL_WIDTH * PIXEL_HEIGHT;

#[embassy_executor::task]
async fn blink(pin: Peri<'static, AnyPin>, interval_ms: u64) {
    let mut led = Output::new(pin, Level::Low, Default::default());

    loop {
        led.set_high();
        Timer::after_millis(interval_ms).await;
        led.set_low();
        Timer::after_millis(interval_ms).await;
    }
}

#[embassy_executor::main(entry = "ch32_hal::entry")]
async fn main(spawner: Spawner) -> ! {
    // init hal, use 144MHz clock with external crystal
    let config = ch32_hal::Config {
        rcc: ch32_hal::rcc::Config::SYSCLK_FREQ_144MHZ_HSE,
        ..Default::default()
    };
    let p = ch32_hal::init(config);

    spawner.spawn(blink(p.PB1.into(), 1000).unwrap());

    let mut leds = Ws2812b::new(p.SPI1, p.PA7, p.DMA1_CH3).await;
    let mut button1 = Button::new_pulldown(p.PA5.into());
    let mut button2 = Button::new_pulldown(p.PA6.into());

    // when booted while holding PA6: use full brightnes instead of automatic dim
    let use_full_color_range = button2.poll().is_down();

    let mut color_buffer = [Color::OFF; PIXEL_COUNT];

    // hot restarts don't clear the LED data, so explicitly clear them here
    let reset_timer = leds.set_colors(&color_buffer, !use_full_color_range).await;
    reset_timer.await;

    // give user a bit of time to restart into bootloader mode without having LEDs on
    Timer::after_millis(500).await;

    let mut effect_index = 0;
    let mut effect = crate::effects::ALL_EFFECTS[effect_index]; // hardcoded for now
    let mut start = Instant::now();
    loop {
        let time = Instant::now() - start;
        effect(time, &mut color_buffer);

        // TODO: don't set colors if they didn't change from last frame
        let reset_timer = leds.set_colors(&color_buffer, !use_full_color_range).await;

        if button1.poll().is_press() {
            effect_index += 1;
            effect_index %= crate::effects::ALL_EFFECTS.len();

            effect = crate::effects::ALL_EFFECTS[effect_index]; // hardcoded for now
            start = Instant::now();
        }

        // NOTE: could generate the next frame while waiting on this to increase framerate
        reset_timer.await;
    }
}
