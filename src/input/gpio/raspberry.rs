use rppal::gpio::{Gpio as RrppalGpio, InputPin};

use super::{Gpio, GpioValue};

pub struct RaspberryGpio {
    active_low: bool,
    pin: InputPin,
}

impl RaspberryGpio {
    /// Create a new [`RaspberryGpio`] instance for the specified GPIO pin
    pub fn try_new(gpio: u8, active_low: bool) -> anyhow::Result<Self> {
        let pin = RrppalGpio::new()
            .map_err(|e| anyhow::anyhow!("Failed to access GPIO: {}", e))?
            .get(gpio)
            .map_err(|e| anyhow::anyhow!("Failed to get GPIO pin {}: {}", gpio, e))?
            .into_input_pullup();

        Ok(RaspberryGpio { active_low, pin })
    }
}

impl Gpio for RaspberryGpio {
    fn read(&mut self) -> anyhow::Result<GpioValue> {
        let value = self.pin.read();
        trace!("Read GPIO {gpio} value: {value}", gpio = self.pin.pin());
        match (value, self.active_low) {
            (rppal::gpio::Level::Low, false) | (rppal::gpio::Level::High, true) => {
                Ok(GpioValue::Disabled)
            }
            (rppal::gpio::Level::High, false) | (rppal::gpio::Level::Low, true) => {
                Ok(GpioValue::Enabled)
            }
        }
    }
}
