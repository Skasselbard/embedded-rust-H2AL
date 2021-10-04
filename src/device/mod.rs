mod stm32f1xx;

use embedded_hal::digital::v2::InputPin;
#[cfg(feature = "stm32f1xx")]
use stm32f1xx as dev;

pub type GpioOutError = dev::GpioOutError;
pub type GpioInError = dev::GpioInError;
pub type Pin = dev::Pin;
pub type Port = dev::Port;
