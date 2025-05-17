#[cfg(feature = "lux_as_f32")]
use crate::calculate_raw_threshold_value;
#[cfg(feature = "lux_as_f32")]
use crate::correction::{correct_high_lux, get_lux_raw_conversion_factor};
use crate::{
    Config, Error, FaultCount, Gain, IntegrationTime, InterruptStatus, PowerSavingMode, Veml7700,
    DEVICE_ADDRESS,
};
use embedded_hal_async::i2c::{ErrorType, I2c, SevenBitAddress};
use maybe_async::maybe_async;

struct Register;
impl Register {
    const ALS_CONF: u8 = 0x00;
    const ALS_WH: u8 = 0x01;
    const ALS_WL: u8 = 0x02;
    const PSM: u8 = 0x03;
    const ALS: u8 = 0x04;
    const WHITE: u8 = 0x05;
    const ALS_INT: u8 = 0x06;
}

struct BitFlags;
impl BitFlags {
    const ALS_SD: u16 = 0x01;
    const ALS_INT_EN: u16 = 0x02;
    const PSM_EN: u16 = 0x01;
    const INT_TH_LOW: u16 = 1 << 15;
    const INT_TH_HIGH: u16 = 1 << 14;
}

impl Config {
    fn with_high(self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl<I2C> Veml7700<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Error<I2C::Error>>,
{
    /// Create new instance of the VEML6040 device.
    pub fn new(i2c: I2C) -> Self {
        Veml7700 {
            i2c,
            config: Config {
                bits: BitFlags::ALS_SD,
            },
            gain: Gain::One,
            it: IntegrationTime::_100ms,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C> Veml7700<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Error<I2C::Error>>,
{
    /// Enable the device.
    ///
    /// Note that when activating the sensor a wait time of 4 ms should be
    /// observed before the first measurement is picked up to allow for a
    /// correct start of the signal processor and oscillator.
    #[maybe_async]
    pub async fn enable(&mut self) -> Result<(), Error<I2C::Error>> {
        let config = self.config.with_low(BitFlags::ALS_SD);
        self.set_config(config).await
    }

    /// Disable the device (shutdown).
    #[maybe_async]
    pub async fn disable(&mut self) -> Result<(), Error<I2C::Error>> {
        let config = self.config.with_high(BitFlags::ALS_SD);
        self.set_config(config).await
    }

    /// Set the integration time.
    #[maybe_async]
    pub async fn set_integration_time(&mut self, it: IntegrationTime) -> Result<(), Error<I2C::Error>> {
        let mask = match it {
            IntegrationTime::_25ms => 0b1100,
            IntegrationTime::_50ms => 0b1000,
            IntegrationTime::_100ms => 0b0000,
            IntegrationTime::_200ms => 0b0001,
            IntegrationTime::_400ms => 0b0010,
            IntegrationTime::_800ms => 0b0011,
        };
        let config = self.config.bits & !(0b1111 << 6) | (mask << 6);
        self.set_config(Config { bits: config }).await?;
        self.it = it;
        Ok(())
    }

    /// Set the gain.
    #[maybe_async]
    pub async fn set_gain(&mut self, gain: Gain) -> Result<(), Error<I2C::Error>> {
        let mask = match gain {
            Gain::One => 0,
            Gain::Two => 1,
            Gain::OneEighth => 2,
            Gain::OneQuarter => 3,
        };
        let config = self.config.bits & !(0b11 << 11) | mask << 11;
        self.set_config(Config { bits: config }).await?;
        self.gain = gain;
        Ok(())
    }

    /// Set the number of times a threshold crossing must happen consecutively
    /// to trigger an interrupt.
    #[maybe_async]
    pub async fn set_fault_count(&mut self, fc: FaultCount) -> Result<(), Error<I2C::Error>> {
        let mask = match fc {
            FaultCount::One => 0,
            FaultCount::Two => 1,
            FaultCount::Four => 2,
            FaultCount::Eight => 3,
        };
        let config = self.config.bits & !(0b11 << 4) | mask << 4;
        self.set_config(Config { bits: config }).await
    }

    /// Enable interrupt generation.
    #[maybe_async]
    pub async fn enable_interrupts(&mut self) -> Result<(), Error<I2C::Error>> {
        let config = self.config.with_high(BitFlags::ALS_INT_EN);
        self.set_config(config).await
    }

    /// Disable interrupt generation.
    #[maybe_async]
    pub async fn disable_interrupts(&mut self) -> Result<(), Error<I2C::Error>> {
        let config = self.config.with_low(BitFlags::ALS_INT_EN);
        self.set_config(config).await
    }

    /// Set the ALS high threshold in raw format
    #[maybe_async]
    pub async fn set_high_threshold_raw(&mut self, threshold: u16) -> Result<(), Error<I2C::Error>> {
        Ok(self.write_register(Register::ALS_WH, threshold).await?)
    }

    /// Set the ALS low threshold in raw format
    #[maybe_async]
    pub async fn set_low_threshold_raw(&mut self, threshold: u16) -> Result<(), Error<I2C::Error>> {
        Ok(self.write_register(Register::ALS_WL, threshold).await?)
    }

    /// Set the ALS high threshold in lux.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the inverse of the compensation formula is applied (this involves
    /// quite some math).
    #[cfg(feature = "lux_as_f32")]
    #[maybe_async]
    pub async fn set_high_threshold_lux(&mut self, lux: f32) -> Result<(), Error<I2C::Error>> {
        let raw = self.calculate_raw_threshold_value(lux);
        self.set_high_threshold_raw(raw).await
    }
    // TODO make a const-able version for pre-calculating the raw-threshold_lux

    /// Set the ALS low threshold in lux.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the inverse of the compensation formula is applied (this involves
    /// quite some math).
    #[cfg(feature = "lux_as_f32")]
    #[maybe_async]
    pub async fn set_low_threshold_lux(&mut self, lux: f32) -> Result<(), Error<I2C::Error>> {
        let raw = self.calculate_raw_threshold_value(lux);
        self.set_low_threshold_raw(raw).await
    }

    /// Calculate raw value for threshold applying compensation if necessary.
    ///
    /// This takes into consideration the configured integration time and gain
    /// and compensates the lux value if necessary.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain, the inverse of the
    /// compensation formula is applied. This involves quite some math so it
    /// may be interesting to calculate the threshold values ahead of time.
    #[cfg(feature = "lux_as_f32")]
    pub fn calculate_raw_threshold_value(&self, lux: f32) -> u16 {
        calculate_raw_threshold_value(self.it, self.gain, lux)
    }

    /// Enable the power-saving mode
    #[maybe_async]
    pub async fn enable_power_saving(&mut self, psm: PowerSavingMode) -> Result<(), Error<I2C::Error>> {
        let mask = match psm {
            PowerSavingMode::One => 0,
            PowerSavingMode::Two => 1,
            PowerSavingMode::Three => 2,
            PowerSavingMode::Four => 3,
        };
        let value = BitFlags::PSM_EN | mask << 1;
        Ok(self.write_register(Register::PSM, value).await?)
    }

    /// Disable the power-saving mode
    #[maybe_async]
    pub async fn disable_power_saving(&mut self) -> Result<(), Error<I2C::Error>> {
        Ok(self.write_register(Register::PSM, 0).await?)
    }

    #[maybe_async]
    async fn set_config(&mut self, config: Config) -> Result<(), Error<I2C::Error>> {
        self.write_register(Register::ALS_CONF, config.bits).await?;
        self.config = config;
        Ok(())
    }

    #[maybe_async]
    async fn write_register(
        &mut self,
        register: u8,
        value: u16,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.i2c
            .write(DEVICE_ADDRESS, &[register, value as u8, (value >> 8) as u8]).await
    }
}

impl<I2C> Veml7700<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Error<I2C::Error>>,
{
    /// Read whether an interrupt has occurred.
    ///
    /// Note that the interrupt status is updated at the same rate as the
    /// measurements. Once triggered, flags will stay true until a measurement
    /// is taken which does not exceed the threshold.
    #[maybe_async]
    pub async fn read_interrupt_status(&mut self) -> Result<InterruptStatus, Error<I2C::Error>> {
        let data = self.read_register(Register::ALS_INT).await?;
        Ok(InterruptStatus {
            was_too_low: (data & BitFlags::INT_TH_LOW) != 0,
            was_too_high: (data & BitFlags::INT_TH_HIGH) != 0,
        })
    }

    /// Read ALS high resolution output data in raw format
    #[maybe_async]
    pub async fn read_raw(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_register(Register::ALS).await
    }

    /// Read ALS high resolution output data converted to lux
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the following compensation formula is applied:
    /// `lux = 6.0135e-13*(lux^4) - 9.3924e-9*(lux^3) + 8.1488e-5*(lux^2) + 1.0023*lux`
    #[cfg(feature = "lux_as_f32")]
    #[maybe_async]
    pub async fn read_lux(&mut self) -> Result<f32, Error<I2C::Error>> {
        let raw = self.read_register(Register::ALS).await?;
        Ok(self.convert_raw_als_to_lux(raw))
    }

    /// Calculate lux value for a raw ALS measurement.
    ///
    /// This takes into consideration the configured integration time and gain
    /// and compensates the lux value if necessary.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the following compensation formula is applied:
    /// `lux = 6.0135e-13*(lux^4) - 9.3924e-9*(lux^3) + 8.1488e-5*(lux^2) + 1.0023*lux`
    #[cfg(feature = "lux_as_f32")]
    pub fn convert_raw_als_to_lux(&self, raw_als: u16) -> f32 {
        convert_raw_als_to_lux(self.it, self.gain, raw_als)
    }

    /// Read white channel measurement
    #[maybe_async]
    pub async fn read_white(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_register(Register::WHITE).await
    }

    #[maybe_async]
    async fn read_register(&mut self, register: u8) -> Result<u16, Error<I2C::Error>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data).await
            .map_err(Error::I2C)
            .and(Ok(u16::from(data[0]) | u16::from(data[1]) << 8))
    }
}

/// Calculate lux value for a raw ALS measurement.
///
/// For values higher than 1000 lx and 1/4 or 1/8 gain,
/// the following compensation formula is applied:
/// `lux = 6.0135e-13*(lux^4) - 9.3924e-9*(lux^3) + 8.1488e-5*(lux^2) + 1.0023*lux`
#[cfg(feature = "lux_as_f32")]
pub fn convert_raw_als_to_lux(it: IntegrationTime, gain: Gain, raw_als: u16) -> f32 {
    let factor = get_lux_raw_conversion_factor(it, gain);
    let lux = f32::from(raw_als) * factor;
    if (gain == Gain::OneQuarter || gain == Gain::OneEighth) && lux > 1000.0 {
        correct_high_lux(lux)
    } else {
        lux
    }
}
