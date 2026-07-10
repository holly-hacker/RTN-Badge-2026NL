#![no_std]
#![no_main]

mod ws2812b;

use ch32_hal::Peri;
use ch32_hal::gpio::{AnyPin, Level, Output};
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;

use crate::ws2812b::Ws2812b;

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

    let mut leds = ws2812b::Ws2812b::new(p.SPI1, p.PA7, p.DMA1_CH3).await;

    let mut color_buffer = [ws2812b::Color::BLACK; 64];

    // hot restarts don't clear the LED data, so explicitly clear them here
    let reset_timer = leds.set_colors(&color_buffer).await;
    reset_timer.await;

    // give user a bit of time to restart int bootloader mode without having LEDs on
    Timer::after_millis(500).await;

    let mut prev_idx = 0;
    let mut cur_idx = 0;
    loop {
        color_buffer[prev_idx] = ws2812b::Color::BLACK;
        color_buffer[cur_idx] = ws2812b::Color::WHITE;

        let reset_timer = leds.set_colors(&color_buffer).await;
        let interval_timer = Timer::after_millis(50);

        // both timers have started, so we're waiting in parallel
        reset_timer.await;
        interval_timer.await;

        prev_idx = cur_idx;
        cur_idx += 1;
        cur_idx %= Ws2812b::COLOR_COUNT;
    }
}
