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
}


pub trait Components {
    fn get_input_pin(&self, index: &InputGpioIndex) -> Result<&InputGpio, ComponentError>;
    fn get_output_pin(&self, index: &OutputGpioIndex) -> Result<&mut OutputGpio, ComponentError>;
}

pub enum ArrayState<const OPCount: usize, const IPCount: usize> {
    Unsorted(ComponentsArrays<OPCount, IPCount>),
    Sorted(ComponentsArrays<OPCount, IPCount>),
}
pub struct ComponentsArrays<const OPCount: usize, const IPCount: usize> {
    outputPins: [Option<OutputGpio>; OPCount],
    inputPins: [Option<InputGpio>; IPCount],
}

impl<const OPCount: usize, const IPCount: usize> ComponentsArrays<OPCount, IPCount> {
    unsafe fn new(memory: &'static mut [u8]) -> Result<Self, ComponentError> {
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
        let outputPins
        todo!()
    }

    fn size_difference(byte_count: usize) -> isize {
        let nedded = (size_of::<Option<InputGpio>>() * IPCount)
            + (size_of::<Option<OutputGpio>>() * OPCount);
        byte_count as isize - nedded as isize
    }
}

impl<const OPCount: usize, const IPCount: usize> ArrayState<OPCount, IPCount> {
    fn new(arrays: ComponentsArrays<OPCount, IPCount>) -> Self {
        ArrayState::Unsorted(arrays)
    }
    fn get_sorted_array(&self) -> Result<&ComponentsArrays<OPCount, IPCount>, ComponentError> {
        match self {
            ArrayState::Unsorted(_) => Err(ComponentError::EarlyAccessAction),
            ArrayState::Sorted(a) => Ok(a),
        }
    }
    fn get_sorted_array_mut(
        &mut self,
    ) -> Result<&mut ComponentsArrays<OPCount, IPCount>, ComponentError> {
        match self {
            ArrayState::Unsorted(_) => Err(ComponentError::EarlyAccessAction),
            ArrayState::Sorted(a) => Ok(a),
        }
    }
    pub fn add_input_pin<T>(&mut self, gpio: &'static mut T) -> Result<(), ComponentError>
    where
        Gpio: FromRef<T>,
        T: InputPin<Error = GpioInError>,
    {
        match self {
            ArrayState::Unsorted(a) => {
                for elem in &mut a.inputPins {
                    if elem.is_none() {
                        elem.replace(InputGpio(Gpio::from_ref(gpio), gpio));
                        return Ok(());
                    }
                }
                Err(ComponentError::OOM)
            }
            ArrayState::Sorted(_) => Err(ComponentError::LateInitAction),
        }
    }
    pub fn get_input_pin(&self, pin: Pin, port: Port) -> Result<InputGpioIndex, ComponentError> {
        let gpio = Gpio { pin, port };
        search_array(&self.get_sorted_array()?.inputPins, &gpio, |v| &v.0)
            .map(|index| InputGpioIndex(index as u8))
    }
    pub fn get_output_pin(&self, pin: Pin, port: Port) -> Result<OutputGpioIndex, ComponentError> {
        let gpio = Gpio { pin, port };
        search_array(&self.get_sorted_array()?.outputPins, &gpio, |v| &v.0)
            .map(|index| OutputGpioIndex(index as u8))
    }
    pub fn finalize(self) -> ArrayState<OPCount, IPCount> {
        match self {
            ArrayState::Unsorted(mut a) => {
                a.outputPins.sort_unstable();
                a.inputPins.sort_unstable();
                ArrayState::Sorted(a)
            }
            ArrayState::Sorted(_) => self,
        }
    }
}

/// Search for an Option encloses value in an array, where the value can be converted into a key with function f
fn search_array<K, V>(
    array: &[Option<V>],
    key: &K,
    f: fn(&V) -> &K,
) -> Result<usize, ComponentError>
where
    K: Ord,
{
    array
        .binary_search_by(|probe| match probe {
            Some(value) => f(value).cmp(key),
            None => Ordering::Less,
        })
        .map_err(|_| ComponentError::NotFound)
}

impl From<GpioError> for ComponentError {
    fn from(e: GpioError) -> Self {
        ComponentError::GpioError(e)
    }
}
