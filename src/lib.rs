#![no_std]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]

mod device;
mod gpio;

use core::cmp::Ordering;
use core::mem::MaybeUninit;
use gpio::InputPin;
use gpio::OutputPin;
use once_cell::unsync::OnceCell;

use device::Pin;
use device::Port;
use gpio::Gpio;
use gpio::GpioError;
use gpio::InputGpio;
use gpio::OutputGpio;

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
/// functions are unsafe because no concurrency safeties are guaranteed.
/// Its your responsibility to synchronize component access.
#[repr(transparent)]
pub struct Components(&'static mut [Component]);

pub struct ComponentsBuilder<const COMPONENT_COUNT: usize> {
    array: &'static mut [MaybeUninit<Component>; COMPONENT_COUNT],
    free_space: usize,
}

impl Component {
    /// The Ordering should be as follows Gpios < None
    /// Gpios are ordered as the Gpio type without respect to Gpio-Kind (In, Out, etc.)
    fn comparator(&self, other: &Self) -> Ordering {
        match other {
            Component::InputGpio(other) => Component::compare_with_gpio(self, &other.0.to_gpio()),
            Component::OutputGpio(other) => Component::compare_with_gpio(self, &other.0.to_gpio()),
        }
    }
    #[inline]
    fn compare_with_gpio(&self, other: &Gpio) -> Ordering {
        match self.to_gpio() {
            Ok(gpio) => gpio.cmp(other),
            Err(_) => Ordering::Greater,
        }
    }
    #[inline]
    fn to_gpio(&self) -> Result<Gpio, ComponentError> {
        match self {
            Component::InputGpio(gpio) => Ok(gpio.0.to_gpio()),
            Component::OutputGpio(gpio) => Ok(gpio.0.to_gpio()),
            _ => Err(ComponentError::ConversionError),
        }
    }
}

impl<const COMPONENT_COUNT: usize> ComponentsBuilder<COMPONENT_COUNT> {
    pub const fn allocate_array() -> [MaybeUninit<Component>; COMPONENT_COUNT] {
        MaybeUninit::uninit_array()
    }
    pub fn new(array: &'static mut [MaybeUninit<Component>; COMPONENT_COUNT]) -> Self {
        Self {
            array,
            free_space: COMPONENT_COUNT,
        }
    }
    fn add_component(&mut self, component: Component) -> Result<(), ComponentError> {
        if self.free_space > 0 {
            self.free_space -= 1;
            self.array[self.free_space].write(component);
            Ok(())
        } else {
            Err(ComponentError::OOM)
        }
    }
    pub fn add_input_pin(&mut self, gpio: &'static mut dyn InputPin) -> Result<(), ComponentError> {
        self.add_component(Component::InputGpio(InputGpio(gpio)))
    }
    pub fn add_output_pin(
        &mut self,
        gpio: &'static mut dyn OutputPin,
    ) -> Result<(), ComponentError> {
        self.add_component(Component::OutputGpio(OutputGpio(gpio)))
    }
    pub unsafe fn finalize(self) -> Result<&'static Components, ()> {
        if self.free_space > 0 {
            // the array has to be initialized completely
            return Err(());
        }
        let array = MaybeUninit::slice_assume_init_mut(self.array);
        array.sort_unstable_by(|this, other| Component::comparator(this, other));
        Components::static_array()
            .set(array)
            .map_err(|_| Err::<(), ()>(()))
            .expect("Multiple Component initialization");
        Ok(Components::get())
    }
}

impl Components {
    unsafe fn static_array() -> &'static mut OnceCell<&'static mut [Component]> {
        static mut ARRAY: OnceCell<&mut [Component]> = OnceCell::new();
        &mut ARRAY
    }
    unsafe fn get() -> &'static mut Self {
        let array = Self::static_array()
            .get_mut()
            .expect("Tried to access uninitialized Components");
        // its the same pointer as the array pointer since the type representation of Self is 'transparent'
        // https://doc.rust-lang.org/1.41.1/reference/type-layout.html#representations
        &mut *(*array as *mut [Component] as *mut Self)
    }
    pub unsafe fn get_input_pin(
        pin: Pin,
        port: Port,
    ) -> Result<&'static mut dyn InputPin, ComponentError> {
        let gpio = Gpio { pin, port };
        let index = Self::search_array(&gpio, Component::compare_with_gpio)?;
        // check if the gpio kind actually matches
        match &mut Self::get().0[index] {
            Component::InputGpio(gpio) => Ok(gpio.0),
            _ => Err(ComponentError::NotFound),
        }
    }
    pub unsafe fn get_output_pin(
        pin: Pin,
        port: Port,
    ) -> Result<&'static mut dyn OutputPin, ComponentError> {
        let gpio = Gpio { pin, port };
        let index = Self::search_array(&gpio, Component::compare_with_gpio)?;
        // check if the gpio kind actually matches
        match &mut Self::get().0[index] {
            Component::OutputGpio(gpio) => Ok(gpio.0),
            _ => Err(ComponentError::NotFound),
        }
    }
    /// Search for a key in the component array, with a function f that can compare the key with components
    unsafe fn search_array<K>(
        key: &K,
        f: fn(&Component, &K) -> Ordering,
    ) -> Result<usize, ComponentError> {
        Self::get()
            .0
            .binary_search_by(|value| f(value, key))
            .map_err(|_| ComponentError::NotFound)
    }
}
