#![no_std]
#![no_main]

mod color;
mod effects;
mod hardware;

use ch32_hal::Peri;
use ch32_hal::gpio::{AnyPin, Level, Output};
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use panic_halt as _;

use crate::color::Color;
use crate::hardware::Ws2812b;

/// The amount of LEDs on the board
pub const COLOR_COUNT: usize = 8 * 8;

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
    let config = ch32_hal::Config {
        rcc: ch32_hal::rcc::Config::SYSCLK_FREQ_144MHZ_HSE,
        ..Default::default()
    };
    let p = ch32_hal::init(config);

    spawner.spawn(blink(p.PB1.into(), 1000).unwrap());

    let mut leds = Ws2812b::new(p.SPI1, p.PA7, p.DMA1_CH3).await;

    let mut color_buffer = [Color::BLACK; COLOR_COUNT];

    // hot restarts don't clear the LED data, so explicitly clear them here
    let reset_timer = leds.set_colors(&color_buffer).await;
    reset_timer.await;

    // give user a bit of time to restart into bootloader mode without having LEDs on
    Timer::after_millis(500).await;

    let effect = crate::effects::ALL_EFFECTS[1]; // hardcoded for now
    let start = Instant::now();
    let mut frame = 0;
    loop {
        let time = Instant::now() - start;
        effect(time, frame, &mut color_buffer);

        let reset_timer = leds.set_colors(&color_buffer).await;

        // NOTE: could generate the next frame while waiting on this to increase framerate
        reset_timer.await;

        frame += 1;
    }
}
