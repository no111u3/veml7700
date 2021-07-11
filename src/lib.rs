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

mod device_impl;
mod types;

pub use crate::types::{FaultCount, Gain, IntegrationTime, InterruptStatus, PowerSavingMode};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

const DEVICE_ADDRESS: u8 = 0x10;

/// VEML7700 device driver.
#[derive(Debug)]
pub struct Veml7700<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    config: Config,
    gain: Gain,
    it: IntegrationTime,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u16,
}
