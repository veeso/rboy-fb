mod linux;
#[cfg(test)]
mod mock;
mod raspberry;

pub use self::linux::LinuxGpio;
#[cfg(test)]
#[allow(unused)]
pub use self::mock::MockGpio;
pub use self::raspberry::RaspberryGpio;

/// GPIO value representation.
///
/// ## Naming
///
/// We do not use "High" and "Low" naming here because
/// the meaning of high/low depends on whether the GPIO is active low or active high.
///
/// So if the GPIO is enabled it means the key/switch is active, regardless of the electrical level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioValue {
    Disabled = 0,
    Enabled = 1,
}

/// GPIO trait abstraction
pub trait Gpio {
    /// Read the current GPIO value
    fn read(&mut self) -> anyhow::Result<GpioValue>;
}
