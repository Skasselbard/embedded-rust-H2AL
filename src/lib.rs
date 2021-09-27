#![no_std]

mod device;
mod gpio;

use core::cmp::Ordering;
use once_cell::unsync::OnceCell;

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
    GpioError(GpioError),
    NotEnoughMemory,
    ConversionError,
}

#[derive(PartialEq, Eq)]
pub enum Component {
    InputGpio(InputGpio),
    OutputGpio(OutputGpio),
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, PartialOrd, Ord, Hash)]
struct ComponentIndex(pub(crate) u8);
#[repr(transparent)]
pub struct Components(&'static mut [Option<Component>]);

pub struct ComponentsBuilder(&'static mut [Option<Component>]);

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

impl ComponentsBuilder {
    pub fn new(array: &'static mut [Option<Component>]) -> Self {
        ComponentsBuilder(array)
    }
    pub fn add_input_pin<T>(&mut self, gpio: &'static mut T) -> Result<(), ComponentError>
    where
        Gpio: FromRef<T>,
        T: InputPin<Error = GpioInError>,
    {
        for elem in self.0.iter_mut() {
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
        for elem in self.0.iter_mut() {
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
    pub fn finalize(self) -> &'static mut Components {
        self.0
            .sort_unstable_by(|this, other| Component::comperator(this, other));
        Components::static_array()
            .set(self.0)
            .map_err(|_| Err::<(), ()>(()))
            .expect("Multible Component initialization");
        Components::get_static()
    }
}

impl Components {
    fn static_array() -> &'static mut OnceCell<&'static mut [Option<Component>]> {
        static mut ARRAY: OnceCell<&mut [Option<Component>]> = OnceCell::new();
        unsafe { &mut ARRAY }
    }
    fn get_static() -> &'static mut Self {
        let array = Self::static_array()
            .get_mut()
            .expect("Tried to access uninitialized Components");
        // its the same pointer as the array pointer since the type representation of Self is 'transparent'
        // https://doc.rust-lang.org/1.41.1/reference/type-layout.html#representations
        unsafe { &mut *(*array as *mut [Option<Component>] as *mut Self) }
    }
    fn get(index: ComponentIndex) -> Result<&'static mut Component, ComponentError> {
        match Self::get_static().0[index.0 as usize].as_mut() {
            Some(c) => Ok(c),
            None => Err(ComponentError::NotFound),
        }
    }
    pub fn get_input_pin(pin: Pin, port: Port) -> Result<InputGpioIndex, ComponentError> {
        let gpio = Gpio { pin, port };
        let index = Self::search_array(&gpio, Component::compare_with_gpio)?;
        // check if the gpio kind actually matches
        match Self::get_static().0[index] {
            Some(Component::InputGpio(_)) => Ok(InputGpioIndex(ComponentIndex(index as u8))),
            Some(_) => Err(ComponentError::NotFound),
            None => Err(ComponentError::NotFound),
        }
    }
    pub fn get_output_pin(pin: Pin, port: Port) -> Result<OutputGpioIndex, ComponentError> {
        let gpio = Gpio { pin, port };
        let index = Self::search_array(&gpio, Component::compare_with_gpio)?;
        // check if the gpio kind actually matches
        match Self::get_static().0[index] {
            Some(Component::OutputGpio(_)) => Ok(OutputGpioIndex(ComponentIndex(index as u8))),
            Some(_) => Err(ComponentError::NotFound),
            None => Err(ComponentError::NotFound),
        }
    }
    /// Search for a key in the component array, with a functin f that can conmpare the key with componetns
    fn search_array<K>(
        key: &K,
        f: fn(&Option<Component>, &K) -> Ordering,
    ) -> Result<usize, ComponentError> {
        Self::get_static()
            .0
            .binary_search_by(|value| f(value, key))
            .map_err(|_| ComponentError::NotFound)
    }
}
