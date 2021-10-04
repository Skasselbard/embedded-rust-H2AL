use embedded_hal::digital::v2;

use crate::{
    device::{GpioInError, GpioOutError, Pin, Port},
    ComponentError,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum GpioError {
    In(GpioInError),
    Out(GpioOutError),
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord, Hash)]
pub struct Gpio {
    pub(super) port: Port,
    pub(super) pin: Pin,
}

pub trait ToGpio {
    fn to_gpio(&self) -> Gpio;
}
pub trait InputPin: v2::InputPin<Error = GpioInError> + ToGpio {}
pub trait OutputPin: v2::OutputPin<Error = GpioOutError> + ToGpio {}

#[repr(transparent)]
pub struct InputGpio(pub(crate) &'static mut dyn InputPin);
#[repr(transparent)]
pub struct OutputGpio(pub(crate) &'static mut dyn OutputPin);

// implement PartialEq, Eq, PartialOrd, Ord for $gpio_type
macro_rules! implement_gpio_cmp_traits {
    ($gpio_type:ident) => {
        impl PartialEq for $gpio_type {
            fn eq(&self, other: &Self) -> bool {
                self.0.to_gpio() == other.0.to_gpio()
            }
        }
        impl Eq for $gpio_type {}
        impl PartialOrd for $gpio_type {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.0.to_gpio().partial_cmp(&other.0.to_gpio())
            }
        }
        impl Ord for $gpio_type {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.to_gpio().cmp(&other.0.to_gpio())
            }
        }
    };
}

implement_gpio_cmp_traits!(InputGpio);
implement_gpio_cmp_traits!(OutputGpio);

impl From<GpioError> for ComponentError {
    fn from(e: GpioError) -> Self {
        ComponentError::GpioError(e)
    }
}
