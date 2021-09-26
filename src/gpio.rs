use embedded_hal::digital::{
    v2::{InputPin, OutputPin},
    
};

use crate::{
    device::{GpioInError, GpioOutError, Pin, Port},
    ComponentError, Components};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum GpioError {
    In(GpioInError),
    Out(GpioOutError),
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct InputGpioIndex(pub(crate) u8);
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct OutputGpioIndex(pub(crate) u8);
#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord, Hash)]
pub struct Gpio {
    pub(super) port: Port,
    pub(super) pin: Pin,
}

pub struct InputGpio(
    pub(crate) Gpio,
    pub(crate) &'static mut dyn InputPin<Error = GpioInError>,
);
pub struct OutputGpio(
    pub(crate) Gpio,
    pub(crate) &'static mut dyn OutputPin<Error = GpioOutError>,
);

impl InputGpioIndex {
    pub fn is_high(&self, components: &dyn Components) -> Result<bool, ComponentError> {
        components
            .get_input_pin(self)?
            .1
            .is_high()
            .map_err(|e| GpioError::In(e).into())
    }

    pub fn is_low(&self, components: &dyn Components) -> Result<bool, ComponentError> {
        components
            .get_input_pin(self)?
            .1
            .is_low()
            .map_err(|e| GpioError::In(e).into())
    }
}

impl OutputGpioIndex {
    pub fn set_high(&self, components: &dyn Components) -> Result<(), ComponentError> {
        components
            .get_output_pin(self)?
            .1
            .set_high()
            .map_err(|e| GpioError::Out(e).into())
    }

    pub fn set_low(&self, components: &dyn Components) -> Result<(), ComponentError> {
        components
            .get_output_pin(self)?
            .1
            .set_low()
            .map_err(|e| GpioError::Out(e).into())
    }
}

// implement PartialEq, Eq, PartialOrd, Ord for $gpio_type
macro_rules! implement_gpio_cmp_traits {
    ($gpio_type:ident) => {
        impl PartialEq for $gpio_type {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl Eq for $gpio_type {}
        impl PartialOrd for $gpio_type {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl Ord for $gpio_type {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
    };
}

implement_gpio_cmp_traits!(InputGpio);
implement_gpio_cmp_traits!(OutputGpio);
