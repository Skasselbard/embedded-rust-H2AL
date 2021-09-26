mod stm32f1xx;

use embedded_hal::digital::v2::InputPin;
#[cfg(feature = "stm32f1xx")]
use stm32f1xx as dev;

use crate::gpio::InputGpioIndex;

pub type GpioOutError = dev::GpioOutError;
pub type GpioInError = dev::GpioInError;
pub type Pin = dev::Pin;
pub type Port = dev::Port;

// muss von device gpio nach abstract gpio und umgekehrt übersetzt werden können
// muss außerdem interrupt kompatibel sein
pub trait OwnInputPin: InputPin {
    fn to_gpio() -> InputGpioIndex;
}
