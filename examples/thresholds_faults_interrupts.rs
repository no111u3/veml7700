#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    delay::Delay,
    i2c::I2c,
    prelude::*,
    serial::{config::Config, Serial},
    stm32,
};

use core::fmt::Write;

use veml7700::{FaultCount, Gain, IntegrationTime, Veml7700};

#[entry]
fn main() -> ! {
    let cp = Peripherals::take().unwrap();

    let p = stm32::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let tx = gpioa.pa2.into_alternate_af7();
    let rx = gpioa.pa3.into_alternate_af7();

    let config = Config::default().baudrate(4_800.bps());

    let serial = Serial::usart2(p.USART2, (tx, rx), config, clocks).unwrap();

    let (mut tx, _rx) = serial.split();

    let gpiob = p.GPIOB.split();
    let scl = gpiob
        .pb8
        .into_alternate_af4()
        .internal_pull_up(true)
        .set_open_drain();

    let sda = gpiob
        .pb9
        .into_alternate_af4()
        .internal_pull_up(true)
        .set_open_drain();

    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 200.khz(), clocks);

    writeln!(tx, "Ambient light sensor from Nucleo F401RE\r").ok();

    // Initialize the VEML7700 with the I2C
    let mut veml7700_device = Veml7700::new(i2c);

    let mut delay = Delay::new(cp.SYST, clocks);

    veml7700_device.set_gain(Gain::OneQuarter).unwrap();
    veml7700_device
        .set_integration_time(IntegrationTime::_200ms)
        .unwrap();
    // this will compensate the value automatically before setting it
    veml7700_device.set_high_threshold_lux(10000.0).unwrap();
    veml7700_device.set_low_threshold_lux(100.0).unwrap();
    veml7700_device.set_fault_count(FaultCount::Four).unwrap();
    veml7700_device.enable_interrupts().unwrap();
    veml7700_device.enable().unwrap();

    loop {
        led.set_high().ok();
        // current light state in lux and white light state
        let white = veml7700_device.read_white().unwrap();
        let lux = veml7700_device.read_lux().unwrap();
        writeln!(tx, "White: {}, Lux: {:2}\r", white, lux).ok();
        let status = veml7700_device.read_interrupt_status().unwrap();
        if status.was_too_high {
            writeln!(tx, "Too high ambient\r").ok();
        }
        if status.was_too_low {
            writeln!(tx, "To low ambient\r").ok();
        }
        led.set_low().ok();
        delay.delay_ms(100u16);
    }
}
