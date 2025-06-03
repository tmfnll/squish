#![no_std]

use esp_hal::analog::adc::RegisterAccess;
use esp_hal::analog::adc::{Adc, AdcChannel, AdcConfig, AdcPin, Attenuation};
use esp_hal::gpio::AnalogPin;
use esp_hal::peripheral::Peripheral;
use esp_hal::Blocking;
use nb::block;

pub struct AnalogInput<'a, ADCI, PIN> {
    adc_pin: AdcPin<PIN, ADCI>,
    adc: Adc<'a, ADCI, Blocking>,
}

impl<'a, ADCI, PIN> AnalogInput<'a, ADCI, PIN>
where
    PIN: AdcChannel + AnalogPin,
    ADCI: RegisterAccess + 'a,
{
    pub fn new(analog_pin: PIN, adc_instance: impl Peripheral<P = ADCI> + 'a) -> Self {
        let mut adc_config = AdcConfig::new();

        let adc_pin = adc_config.enable_pin(analog_pin, Attenuation::_11dB);

        let adc = Adc::new(adc_instance, adc_config);

        Self { adc_pin, adc }
    }

    pub fn read(&mut self) -> u16 {
        block!(self.adc.read_oneshot(&mut self.adc_pin)).unwrap()
    }
}
