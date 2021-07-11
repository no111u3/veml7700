//! This is a platform-agnostic Rust driver for the VEML7700 High Accuracy
//! Ambient Light Sensor, based on the [`embedded-hal`] traits.
//!
//! # The device
//!
//! VEML7700 is a high accuracy ambient light digital 16-bit
//! resolution sensor in a miniature transparent 6.8 mm x
//! 2.35 mm x 3.0 mm package. It includes a high sensitive
//! photo diode, a low noise amplifier, a 16-bit A/D converter
//! and supports an easy to use I2C bus communication
//! interface.
//! The ambient light result is as digital value available.
//!
//! Datasheet: [VEML7700](https://www.vishay.com/docs/84286/veml7700.pdf)
//!
//! Application note: [VEML7700 AN](https://www.vishay.com/docs/84323/designingveml7700.pdf)
#![deny(unsafe_code)]
#![no_std]

use embedded_hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Integration time
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegrationTime {
    /// 25 ms
    _25ms,
    /// 50 ms
    _50ms,
    /// 100 ms
    _100ms,
    /// 200 ms
    _200ms,
    /// 400 ms
    _400ms,
    /// 800 ms
    _800ms,
}

/// Result of measurement of all channels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllChannelMeasurement {
    /// ALS channel measurement.
    pub als: u16,
    /// White channel measurement.
    pub white: u16,
}

const DEVICE_ADDRESS: u8 = 0x10;

struct Register;

impl Register {
    const CONFIG: u8 = 0x00;
    const ALS_WINDOW_HIGH: u8 = 0x01;
    const ALS_WINDOW_LOW: u8 = 0x02;
    const POWER_SAVE: u8 = 0x03;
    const ALS: u8 = 0x04;
    const WHITE: u8 = 0x05;
    const ALS_INT: u8 = 0x06;
}

/// VEML7700 device driver.
#[derive(Debug, Default)]
pub struct Veml7700<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}

impl<I2C, E> Veml7700<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the VEML6040 device.
    pub fn new(i2c: I2C) -> Self {
        Veml7700 { i2c }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
