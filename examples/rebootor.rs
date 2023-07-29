// This example demonstrates a program that also acts as a rebootor.
// When being programmed with the `-r` flag, it will reboot itself into
// the bootloader.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;

use bsp::board;
use bsp::hal;

mod common;
use common::uart::{uart_log, UartWriter};

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        mut gpio2,
        pins,
        lpuart6,
        gpt1: mut us_timer,
        //ccm,
        ..
    } = board::tmm(board::instances());

    // Initialize LED
    let led = board::led(&mut gpio2, pins.p13);
    led.set();

    // Initialize UART
    let mut uart = UartWriter::new(board::lpuart(lpuart6, pins.p1, pins.p0, 115200));
    writeln!(uart);

    // Write welcome message
    writeln!(uart, "===== Rebootor example =====");
    writeln!(uart);

    // Initialize logging
    uart_log::init(uart, log::LevelFilter::Debug);

    // Initialize timer
    // Is a 32-bit timer with us precision.
    // Overflows every 71.58 minutes, which is sufficient for our example.
    log::info!("Initializing timer ...");
    assert_eq!(board::PERCLK_FREQUENCY, 1_000_000);
    us_timer.set_clock_source(hal::gpt::ClockSource::PeripheralClock);
    us_timer.set_divider(1);
    us_timer.set_mode(hal::gpt::Mode::FreeRunning);
    us_timer.enable();
    let time_us = move || us_timer.count();
    log::debug!("Timer initialized.");

    const PERIOD: u32 = 100_000; // microseconds

    let mut last_update = time_us();
    loop {
        let now = time_us();
        let diff = now.wrapping_sub(last_update);

        if diff > PERIOD {
            last_update = now;

            led.toggle();
        }
    }
}
