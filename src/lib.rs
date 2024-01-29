//! This is a platform agnostic Rust driver for the Veml7700 and VEML7700 high-accuracy
//! ambient light sensors using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device. See: [`enable()`].
//! - Read the measured lux value. See: [`read_lux()`].
//! - Read the white channel measurement. See: [`read_white()`].
//! - Read the measured ALS value in raw format. See: [`read_raw()`].
//! - Calculate the compensated lux for a raw ALS value. See: [`convert_raw_als_to_lux()`].
//! - Set the gain. See: [`set_gain()`].
//! - Set the integration time. See: [`set_integration_time()`].
//! - Set the fault count. See: [`set_fault_count()`].
//! - Enable/disable and configure power saving mode. See: [`enable_power_saving()`].
//! - Enable/disable interrupts. See: [`enable_interrupts()`].
//! - Read the interrupt status. See: [`read_interrupt_status()`].
//! - Set the high/low thresholds in lux or raw. See: [`set_high_threshold_lux()`].
//! - Calculate the compensated raw threshold value ahead of time. See: [`calculate_raw_threshold_value()`].
//!
//! [`enable()`]: struct.Veml7700.html#method.enable
//! [`read_lux()`]: struct.Veml7700.html#method.read_lux
//! [`read_white()`]: struct.Veml7700.html#method.read_white
//! [`read_raw()`]: struct.Veml7700.html#method.read_raw
//! [`convert_raw_als_to_lux()`]: fn.convert_raw_als_to_lux.html
//! [`set_gain()`]: struct.Veml7700.html#method.set_gain
//! [`set_integration_time()`]: struct.Veml7700.html#method.set_integration_time
//! [`set_fault_count()`]: struct.Veml7700.html#method.set_fault_count
//! [`enable_power_saving()`]: struct.Veml7700.html#method.enable_power_saving
//! [`enable_interrupts()`]: struct.Veml7700.html#method.enable_interrupts
//! [`read_interrupt_status()`]: struct.Veml7700.html#method.read_interrupt_status
//! [`set_high_threshold_lux()`]: struct.Veml7700.html#method.set_high_threshold_lux
//! [`calculate_raw_threshold_value()`]: fn.calculate_raw_threshold_value.html
//!
//! ## The device
//!
//! Vishay'sVEML7700 are high accuracy ambient light digital 16-bit
//! resolution sensor in a miniature transparent package. It includes
//! a high sensitive photodiode, a low noise amplifier, a 16-bit A/D converter
//! and support an easy to use I2C bus communication interface and additional
//! interrupt feature.
//! The ambient light result is as digital value available.
//!
//! Datasheet: [VEML7700](https://www.vishay.com/docs/84286/veml7700.pdf)
//!
//! Application Note:
//! - [Designing the VEML7700 into an application](https://www.vishay.com/docs/84323/designingveml7700.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! VEML7700 expose interface over I2C.
#![deny(unsafe_code, missing_docs)]
#![no_std]

#[cfg(feature = "lux_as_f32")]
mod correction;

mod device_impl;
mod types;

#[cfg(feature = "lux_as_f32")]
pub use crate::correction::calculate_raw_threshold_value;
#[cfg(feature = "lux_as_f32")]
pub use crate::device_impl::convert_raw_als_to_lux;

pub use crate::types::{FaultCount, Gain, IntegrationTime, InterruptStatus, PowerSavingMode};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}
impl<E> From<E> for Error<E> {
    fn from(other: E) -> Self {
        Error::I2C(other)
    }
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
