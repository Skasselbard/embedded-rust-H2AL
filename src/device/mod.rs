mod stm32f1xx;

#[cfg(feature = "stm32f1xx")]
use stm32f1xx as dev;

pub type GpioOutError = dev::GpioOutError;
pub type GpioInError = dev::GpioInError;
pub type TimerError = dev::TimerError;
pub type Pin = dev::Pin;
pub type Port = dev::Port;
pub type TimerID = dev::TimerID;
pub type Time = dev::Time;

#[inline]
pub(crate) fn disable_interrupts() {
    dev::disable_interrupts()
}
#[inline]
pub(crate) fn enable_interrupts() {
    dev::enable_interrupts()
}
