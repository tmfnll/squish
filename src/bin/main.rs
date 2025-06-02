//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO4)

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use log::info;
use squish::LinearPotentiometer;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_println::logger::init_logger_from_env();

    let mut led = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());

    let mut potentiometer = LinearPotentiometer::new(peripherals.GPIO5, peripherals.ADC1);

    let delay = Delay::new();

    let mut pin_value: u16;
    let mut old_pin_value: u16 = 1;

    loop {
        delay.delay_millis(1);

        pin_value = potentiometer.read();

        if pin_value > 3000 {
            led.set_high();
        } else {
            led.set_low();
        }

        if pin_value != old_pin_value {
            info!("Potentiometer value: {}", pin_value);
        }

        old_pin_value = pin_value;
    }
}
