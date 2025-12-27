use std::path::Path;

use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

use super::{Gpio, GpioValue};

pub struct LinuxGpio {
    handle: LineHandle,
}

impl LinuxGpio {
    /// Create a new [`LinuxGpio`] instance for the specified GPIO pin
    pub fn try_new(device: &Path, gpio: u8, active_low: bool) -> anyhow::Result<Self> {
        debug!("Opening chip at {:?}", device);
        let mut chip = Chip::new(device)
            .map_err(|e| anyhow::anyhow!("Failed to open GPIO chip {:?}: {}", device, e))?;
        debug!("Requesting line for GPIO {}", gpio);
        let line = chip.get_line(gpio as u32)?;
        debug!("Configuring line for GPIO {gpio}");

        let mut flags = LineRequestFlags::INPUT;
        if active_low {
            flags |= LineRequestFlags::ACTIVE_LOW;
            debug!("Setting line ACTIVE_LOW for GPIO {gpio}");
        }

        // request handle
        debug!("Requesting line handle for GPIO {gpio}");
        let handle = line.request(flags, 0, "gpio2key")?;

        Ok(LinuxGpio { handle })
    }
}

impl Gpio for LinuxGpio {
    fn read(&mut self) -> anyhow::Result<GpioValue> {
        let value = self.handle.get_value()?;
        trace!(
            "Read GPIO {gpio} value: {value}",
            gpio = self.handle.line().offset()
        );
        match value {
            0 => Ok(GpioValue::Disabled),
            1 => Ok(GpioValue::Enabled),
            v => Err(anyhow::anyhow!("Unexpected GPIO value: {}", v)),
        }
    }
}
