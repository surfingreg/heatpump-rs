//! led.rs
//! //! https://github.com/olegccc/esp32-axum-ws/blob/main/.cargo/config.toml
//!

use anyhow::Result;
use esp_idf_svc::hal::gpio::{Gpio48, Output, PinDriver, Pins};

pub struct Led {
    led_pin: PinDriver<'static, Gpio48, Output>
}

impl Led {
    pub fn load(pins: Pins) -> Result<Self> {
        let p48 = PinDriver::output(pins.gpio48)?;
        let mut ret = Self {
            led_pin: p48,
        };
        ret.off()?;
        Ok(ret)
    }
    
    pub fn on(&mut self) -> Result<()> {
        self.led_pin.set_low()?;
        Ok(())
    }
    
    pub fn off(&mut self) -> Result<()> {
        self.led_pin.set_high()?;
        Ok(())
    }
}
