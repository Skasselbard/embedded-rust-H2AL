use embedded_hal::digital::v2;

use crate::{
    device::{GpioInError, GpioOutError, Pin, Port},
    ComponentError, ComponentIndex, Components,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum GpioError {
    In(GpioInError),
    Out(GpioOutError),
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct InputGpioIndex(pub(crate) ComponentIndex);

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct OutputGpioIndex(pub(crate) ComponentIndex);

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
pub struct OutputGpio(pub(crate) &'static mut dyn OutputPin<Error = GpioOutError>);

impl InputGpioIndex {
    pub unsafe fn is_high(&self) -> Result<bool, ComponentError> {
        match Components::get(self.0)? {
            crate::Component::InputGpio(gpio) => {
                gpio.0.is_high().map_err(|e| GpioError::In(e).into())
            }
            _ => Err(ComponentError::NotFound),
        }
    }

    pub unsafe fn is_low(&self) -> Result<bool, ComponentError> {
        match Components::get(self.0)? {
            crate::Component::InputGpio(gpio) => {
                gpio.0.is_low().map_err(|e| GpioError::In(e).into())
            }
            _ => Err(ComponentError::NotFound),
        }
    }
}

impl OutputGpioIndex {
    pub unsafe fn set_high(&self) -> Result<(), ComponentError> {
        match Components::get(self.0)? {
            crate::Component::OutputGpio(gpio) => {
                gpio.0.set_high().map_err(|e| GpioError::In(e).into())
            }
            _ => Err(ComponentError::NotFound),
        }
    }

    pub unsafe fn set_low(&self) -> Result<(), ComponentError> {
        match Components::get(self.0)? {
            crate::Component::OutputGpio(gpio) => {
                gpio.0.set_low().map_err(|e| GpioError::In(e).into())
            }
            _ => Err(ComponentError::NotFound),
        }
    }
}

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
