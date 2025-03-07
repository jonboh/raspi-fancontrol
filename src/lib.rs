use anyhow::{anyhow, Result};
use rppal::pwm::{Channel, Polarity};
use std::time::Instant;

pub struct FanHardwareConfig {
    pub freq: f64,
    pub pwm_channel: Channel,
    pub polarity: Polarity,
}
pub struct TachoHardwareConfig {
    pub pulse_per_revolution: u8,
    pub gpio: u8,
}

impl FanHardwareConfig {
    /// Previous to rpi5
    /// PWM0 = GPIO12/GPIO18
    /// PWM1 = GPIO13/GPIO19
    /// On rpi5
    /// PWM0 = GPIO12
    /// PWM1 = GPIO13
    /// PWM2 = GPIO18
    /// PWM3 = GPIO19
    // NOTE: see: https://www.noctua.at/pub/media/wysiwyg/Noctua_PWM_specifications_white_paper.pdf
    #[must_use]
    pub const fn noctua_fan(pwm_channel: Channel) -> Self {
        Self {
            freq: 25_000f64,
            pwm_channel,
            polarity: Polarity::Normal,
        }
    }
}

impl TachoHardwareConfig {
    #[must_use]
    pub const fn noctua_tacho(gpio: u8) -> Self {
        Self {
            pulse_per_revolution: 2,
            gpio,
        }
    }
}

pub struct Tacho {
    counter: u64,
    last_query: std::time::Instant,
    config: TachoHardwareConfig,
}

impl Tacho {
    pub fn new(config: TachoHardwareConfig) -> Self {
        Self {
            counter: 0,
            last_query: Instant::now(),
            config,
        }
    }
    pub fn get_rpm(&mut self) -> Result<f64> {
        let now = Instant::now();
        let elapsed = now - self.last_query;
        let frequency = dbg!(self.counter) as f64 / elapsed.as_secs() as f64;
        let rpm = frequency * 60.0 / self.config.pulse_per_revolution as f64;
        self.counter = 0;
        self.last_query = now;
        Ok(rpm)
    }

    pub fn handle_interrupt(&mut self) {
        self.counter += 1;
    }
}
impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

impl std::fmt::Display for PWM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct PWM(pub f64);

impl PWM {
    pub fn new(value: f64) -> Result<Self> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(anyhow!("PWM value outside [0, 1] range"))
        }
    }
    #[must_use]
    pub fn new_saturate(value: f64) -> PWM {
        Self(value.clamp(0.0, 1.0))
    }
}
