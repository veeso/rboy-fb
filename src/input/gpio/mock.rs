use super::{Gpio, GpioValue};

/// Mock GPIO implementation for testing
pub struct MockGpio {
    active_low: bool,
    value: bool,
}

impl MockGpio {
    pub fn new(initial_value: bool, active_low: bool) -> Self {
        Self {
            active_low,
            value: initial_value,
        }
    }
}

impl Gpio for MockGpio {
    fn read(&mut self) -> anyhow::Result<GpioValue> {
        let gpio_value = if (self.value && !self.active_low) || (!self.value && self.active_low) {
            GpioValue::Enabled
        } else {
            GpioValue::Disabled
        };
        Ok(gpio_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_gpio() {
        let mut gpio_active_high = MockGpio::new(true, false);
        assert_eq!(gpio_active_high.read().unwrap(), GpioValue::Enabled);

        let mut gpio_active_low = MockGpio::new(true, true);
        assert_eq!(gpio_active_low.read().unwrap(), GpioValue::Disabled);

        let mut gpio_active_high_low = MockGpio::new(false, false);
        assert_eq!(gpio_active_high_low.read().unwrap(), GpioValue::Disabled);

        let mut gpio_active_low_high = MockGpio::new(false, true);
        assert_eq!(gpio_active_low_high.read().unwrap(), GpioValue::Enabled);
    }
}
