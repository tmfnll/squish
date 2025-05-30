//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO4)

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main
};
use log::info;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    esp_println::logger::init_logger_from_env();

    let mut led = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let delay = Delay::new();

    loop {
        info!("Toggling LED...");
        led.toggle();
        delay.delay_millis(500);
    }
}
