# Rust VEML7700 High Accuracy Ambient Light Sensor Driver
[![crates.io](https://img.shields.io/crates/v/veml7700.svg)](https://crates.io/crates/veml7700)
[![Docs](https://docs.rs/veml7700/badge.svg)](https://docs.rs/veml7700)

This is a platform agnostic Rust driver for the VEML7700 high accuracy ambient
light sensors using the [`embedded-hal`] traits. It's alternative version of
[`veml6030`] crate that uses the [`micromath`] library and 32 bit precision for sensor correction.

This driver allows you to:
- Enable/disable the device. See: `enable()`.
- Read the measured lux value. See: `read_lux()`.
- Read the white channel measurement. See: `read_white()`.
- Read the measured ALS value in raw format. See: `read_raw()`.
- Calculate the compensated lux for a raw ALS value. See: `convert_raw_als_to_lux()`.
- Set the gain. See: `set_gain()`.
- Set the integration time. See: `set_integration_time()`.
- Set the fault count. See: `set_fault_count()`.
- Enable/disable and configure power saving mode. See: `enable_power_saving()`.
- Enable/disable interrupts. See: `enable_interrupts()`.
- Read the interrupt status. See: `read_interrupt_status()`.
- Set the high/low thresholds in lux or raw. See: `set_high_threshold_lux()`.
- Calculate the compensated raw threshold value ahead of time. See: `calculate_raw_threshold_value()`.

## The device

Vishay's VEML7700 are high accuracy ambient light digital 16-bit
resolution sensor in a miniature transparent package. It includes
a high sensitive photodiode, a low noise amplifier, a 16-bit A/D converter
and support an easy to use I2C bus communication interface and additional
interrupt feature.
The ambient light result is as digital value available.

Datasheet:[VEML7700](https://www.vishay.com/docs/84286/veml7700.pdf)

Application Note:
- [Designing the VEML7700 into an application](https://www.vishay.com/docs/84323/designingveml7700.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

VEML7700 expose interface over I2C.

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/no111u3/veml7700/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
[`micromath`]: https://github.com/tarcieri/micromath
[`veml6030`]: https://github.com/eldruin/veml6030-rs

