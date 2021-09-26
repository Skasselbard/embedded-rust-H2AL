#![no_std]

mod device;
mod gpio;

use core::cmp::Ordering;
use core::mem::size_of;
use core::mem::size_of_val;

use device::GpioInError;
use device::GpioOutError;
use device::Pin;
use device::Port;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use gpio::Gpio;
use gpio::GpioError;
use gpio::InputGpio;
use gpio::InputGpioIndex;
use gpio::OutputGpio;
use gpio::OutputGpioIndex;

pub trait FromRef<T> {
    fn from_ref(source: &T) -> Self;
}
pub struct EventQueue {}

#[non_exhaustive]
pub enum ComponentError {
    LateInitAction,
    EarlyAccessAction,
    NotFound,
    OOM,
    GpioError(gpio::GpioError),
    NotEnoughMemory,
    ConversionError,
}

#[derive(PartialEq, Eq)]
enum Component {
    InputGpio(InputGpio),
    OutputGpio(OutputGpio),
}
pub trait Components {
    fn get_input_pin(&self, index: &InputGpioIndex) -> Result<&InputGpio, ComponentError>;
    fn get_output_pin(&self, index: &OutputGpioIndex) -> Result<&mut OutputGpio, ComponentError>;
}

pub enum ArrayState<const COMPONENT_COUNT: usize> {
    Unsorted(ComponentsArray<COMPONENT_COUNT>),
    Sorted(ComponentsArray<COMPONENT_COUNT>),
}
pub struct ComponentsArray<const COMPONENT_COUNT: usize>([Option<Component>; COMPONENT_COUNT]);

impl Component {
    /// The Ordering should be as follows Gpios < None
    /// Gpios are ordered as the Gpio type without respect to Gpio-Kind (In, Out, etc.)
    fn comperator(this: &Option<Self>, other: &Option<Self>) -> Ordering {
        match other {
            Some(Component::InputGpio(other)) => Component::compare_with_gpio(this, &other.0),
            Some(Component::OutputGpio(other)) => Component::compare_with_gpio(this, &other.0),
            None => Component::compare_with_none(this),
        }
    }
    #[inline]
    fn compare_with_none(this: &Option<Self>) -> Ordering {
        match this {
            Some(_) => Ordering::Less,
            None => Ordering::Equal,
        }
    }
    #[inline]
    fn compare_with_gpio(this: &Option<Self>, other: &Gpio) -> Ordering {
        match this {
            Some(this) => match this.to_gpio() {
                Ok(gpio) => gpio.cmp(other),
                Err(_) => Ordering::Greater,
            },
            None => Ordering::Greater,
        }
    }
    #[inline]
    fn to_gpio(&self) -> Result<&Gpio, ComponentError> {
        match self {
            Component::InputGpio(gpio) => Ok(&gpio.0),
            Component::OutputGpio(gpio) => Ok(&gpio.0),
            _ => Err(ComponentError::ConversionError),
        }
    }
}

impl<const COMPONENT_COUNT: usize> ComponentsArray<COMPONENT_COUNT> {
    pub unsafe fn new(memory: &'static mut [u8]) -> Result<Self, ComponentError> {
        let size_difference = Self::size_difference(size_of_val(memory));
        if size_difference < 0 {
            return Err(ComponentError::NotEnoughMemory);
        } else {
            if size_difference > 0 {
                log::warn!(
                    "memory for components array is {} bytes to large",
                    size_difference
                );
            }
        }
        
    }

    fn size_difference(byte_count: usize) -> isize {
        let nedded = size_of::<Option<Component>>() * COMPONENT_COUNT;
        byte_count as isize - nedded as isize
    }
}

impl<const COMPONENT_COUNT: usize> ArrayState<COMPONENT_COUNT> {
    pub fn new(arrays: ComponentsArray<COMPONENT_COUNT>) -> Self {
        ArrayState::Unsorted(arrays)
    }
    fn get_sorted_array(&self) -> Result<&ComponentsArray<COMPONENT_COUNT>, ComponentError> {
        match self {
            ArrayState::Unsorted(_) => Err(ComponentError::EarlyAccessAction),
            ArrayState::Sorted(a) => Ok(a),
        }
    }
    fn get_unsorted_array(
        &mut self,
    ) -> Result<&mut ComponentsArray<COMPONENT_COUNT>, ComponentError> {
        match self {
            ArrayState::Sorted(_) => Err(ComponentError::LateInitAction),
            ArrayState::Unsorted(a) => Ok(a),
        }
    }
    pub fn add_input_pin<T>(&mut self, gpio: &'static mut T) -> Result<(), ComponentError>
    where
        Gpio: FromRef<T>,
        T: InputPin<Error = GpioInError>,
    {
        for elem in &mut self.get_unsorted_array()?.0 {
            if elem.is_none() {
                elem.replace(Component::InputGpio(InputGpio(Gpio::from_ref(gpio), gpio)));
                return Ok(());
            }
        }
        Err(ComponentError::OOM)
    }
    pub fn add_output_pin<T>(&mut self, gpio: &'static mut T) -> Result<(), ComponentError>
    where
        Gpio: FromRef<T>,
        T: OutputPin<Error = GpioOutError>,
    {
        for elem in &mut self.get_unsorted_array()?.0 {
            if elem.is_none() {
                elem.replace(Component::OutputGpio(OutputGpio(
                    Gpio::from_ref(gpio),
                    gpio,
                )));
                return Ok(());
            }
        }
        Err(ComponentError::OOM)
    }
    pub fn get_input_pin(&self, pin: Pin, port: Port) -> Result<InputGpioIndex, ComponentError> {
        let gpio = Gpio { pin, port };
        let index = self.search_array(&gpio, Component::compare_with_gpio)?;
        // check if the gpio kind actually matches
        match self.get_sorted_array()?.0[index] {
            Some(Component::InputGpio(_)) => Ok(InputGpioIndex(index as u8)),
            Some(_) => Err(ComponentError::NotFound),
            None => Err(ComponentError::NotFound),
        }
    }
    pub fn get_output_pin(&self, pin: Pin, port: Port) -> Result<OutputGpioIndex, ComponentError> {
        let gpio = Gpio { pin, port };
        let index = self.search_array(&gpio, Component::compare_with_gpio)?;
        // check if the gpio kind actually matches
        match self.get_sorted_array()?.0[index] {
            Some(Component::OutputGpio(_)) => Ok(OutputGpioIndex(index as u8)),
            Some(_) => Err(ComponentError::NotFound),
            None => Err(ComponentError::NotFound),
        }
    }
    /// Search for a key in the component array, with a functin f that can conmpare the key with componetns
    fn search_array<K>(
        &self,
        key: &K,
        f: fn(&Option<Component>, &K) -> Ordering,
    ) -> Result<usize, ComponentError> {
        self.get_sorted_array()?
            .0
            .binary_search_by(|value| f(value, key))
            .map_err(|_| ComponentError::NotFound)
    }
    pub fn finalize(self) -> ArrayState<COMPONENT_COUNT> {
        match self {
            ArrayState::Unsorted(mut a) => {
                a.0.sort_unstable_by(|this, other| Component::comperator(this, other));
                ArrayState::Sorted(a)
            }
            ArrayState::Sorted(_) => self,
        }
    }
}

impl From<GpioError> for ComponentError {
    fn from(e: GpioError) -> Self {
        ComponentError::GpioError(e)
    }
}
