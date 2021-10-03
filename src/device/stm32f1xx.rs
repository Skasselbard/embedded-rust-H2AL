use core::mem::transmute;

use crate::gpio::{Gpio, ToGpio};
use stm32f1xx_hal::gpio;

pub type GpioOutError = core::convert::Infallible;
pub type GpioInError = core::convert::Infallible;

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum Port {
    A,
    B,
    C,
    D,
    E,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum Pin {
    P00,
    P01,
    P02,
    P03,
    P04,
    P05,
    P06,
    P07,
    P08,
    P09,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}

// implements conversion from a hal pin into a Gpio e.g. from stm32f1xx_hal::gpio::gpioa::PA<T>
macro_rules! implement_to_gpio {
    ($port:expr, $pin:expr, $hal_port:tt::$hal_pin:tt) => {
        impl<T> ToGpio for gpio::$hal_port::$hal_pin<T> {
            #[inline]
            fn to_gpio(&self) -> Gpio {
                Gpio {
                    port: $port,
                    pin: $pin,
                }
            }
        }
    };
}

implement_to_gpio!(Port::A, Pin::P00, gpioa::PA0);
implement_to_gpio!(Port::A, Pin::P01, gpioa::PA1);
implement_to_gpio!(Port::A, Pin::P02, gpioa::PA2);
implement_to_gpio!(Port::A, Pin::P03, gpioa::PA3);
implement_to_gpio!(Port::A, Pin::P04, gpioa::PA4);
implement_to_gpio!(Port::A, Pin::P05, gpioa::PA5);
implement_to_gpio!(Port::A, Pin::P06, gpioa::PA6);
implement_to_gpio!(Port::A, Pin::P07, gpioa::PA7);
implement_to_gpio!(Port::A, Pin::P08, gpioa::PA8);
implement_to_gpio!(Port::A, Pin::P09, gpioa::PA9);
implement_to_gpio!(Port::A, Pin::P10, gpioa::PA10);
implement_to_gpio!(Port::A, Pin::P11, gpioa::PA11);
implement_to_gpio!(Port::A, Pin::P12, gpioa::PA12);
implement_to_gpio!(Port::A, Pin::P13, gpioa::PA13);
implement_to_gpio!(Port::A, Pin::P14, gpioa::PA14);
implement_to_gpio!(Port::A, Pin::P15, gpioa::PA15);
implement_to_gpio!(Port::B, Pin::P00, gpiob::PB0);
implement_to_gpio!(Port::B, Pin::P01, gpiob::PB1);
implement_to_gpio!(Port::B, Pin::P02, gpiob::PB2);
implement_to_gpio!(Port::B, Pin::P03, gpiob::PB3);
implement_to_gpio!(Port::B, Pin::P04, gpiob::PB4);
implement_to_gpio!(Port::B, Pin::P05, gpiob::PB5);
implement_to_gpio!(Port::B, Pin::P06, gpiob::PB6);
implement_to_gpio!(Port::B, Pin::P07, gpiob::PB7);
implement_to_gpio!(Port::B, Pin::P08, gpiob::PB8);
implement_to_gpio!(Port::B, Pin::P09, gpiob::PB9);
implement_to_gpio!(Port::B, Pin::P10, gpiob::PB10);
implement_to_gpio!(Port::B, Pin::P11, gpiob::PB11);
implement_to_gpio!(Port::B, Pin::P12, gpiob::PB12);
implement_to_gpio!(Port::B, Pin::P13, gpiob::PB13);
implement_to_gpio!(Port::B, Pin::P14, gpiob::PB14);
implement_to_gpio!(Port::B, Pin::P15, gpiob::PB15);
implement_to_gpio!(Port::C, Pin::P00, gpioc::PC0);
implement_to_gpio!(Port::C, Pin::P01, gpioc::PC1);
implement_to_gpio!(Port::C, Pin::P02, gpioc::PC2);
implement_to_gpio!(Port::C, Pin::P03, gpioc::PC3);
implement_to_gpio!(Port::C, Pin::P04, gpioc::PC4);
implement_to_gpio!(Port::C, Pin::P05, gpioc::PC5);
implement_to_gpio!(Port::C, Pin::P06, gpioc::PC6);
implement_to_gpio!(Port::C, Pin::P07, gpioc::PC7);
implement_to_gpio!(Port::C, Pin::P08, gpioc::PC8);
implement_to_gpio!(Port::C, Pin::P09, gpioc::PC9);
implement_to_gpio!(Port::C, Pin::P10, gpioc::PC10);
implement_to_gpio!(Port::C, Pin::P11, gpioc::PC11);
implement_to_gpio!(Port::C, Pin::P12, gpioc::PC12);
implement_to_gpio!(Port::C, Pin::P13, gpioc::PC13);
implement_to_gpio!(Port::C, Pin::P14, gpioc::PC14);
implement_to_gpio!(Port::C, Pin::P15, gpioc::PC15);
implement_to_gpio!(Port::D, Pin::P00, gpiod::PD0);
implement_to_gpio!(Port::D, Pin::P01, gpiod::PD1);
implement_to_gpio!(Port::D, Pin::P02, gpiod::PD2);
implement_to_gpio!(Port::D, Pin::P03, gpiod::PD3);
implement_to_gpio!(Port::D, Pin::P04, gpiod::PD4);
implement_to_gpio!(Port::D, Pin::P05, gpiod::PD5);
implement_to_gpio!(Port::D, Pin::P06, gpiod::PD6);
implement_to_gpio!(Port::D, Pin::P07, gpiod::PD7);
implement_to_gpio!(Port::D, Pin::P08, gpiod::PD8);
implement_to_gpio!(Port::D, Pin::P09, gpiod::PD9);
implement_to_gpio!(Port::D, Pin::P10, gpiod::PD10);
implement_to_gpio!(Port::D, Pin::P11, gpiod::PD11);
implement_to_gpio!(Port::D, Pin::P12, gpiod::PD12);
implement_to_gpio!(Port::D, Pin::P13, gpiod::PD13);
implement_to_gpio!(Port::D, Pin::P14, gpiod::PD14);
implement_to_gpio!(Port::D, Pin::P15, gpiod::PD15);
implement_to_gpio!(Port::E, Pin::P00, gpioe::PE0);
implement_to_gpio!(Port::E, Pin::P01, gpioe::PE1);
implement_to_gpio!(Port::E, Pin::P02, gpioe::PE2);
implement_to_gpio!(Port::E, Pin::P03, gpioe::PE3);
implement_to_gpio!(Port::E, Pin::P04, gpioe::PE4);
implement_to_gpio!(Port::E, Pin::P05, gpioe::PE5);
implement_to_gpio!(Port::E, Pin::P06, gpioe::PE6);
implement_to_gpio!(Port::E, Pin::P07, gpioe::PE7);
implement_to_gpio!(Port::E, Pin::P08, gpioe::PE8);
implement_to_gpio!(Port::E, Pin::P09, gpioe::PE9);
implement_to_gpio!(Port::E, Pin::P10, gpioe::PE10);
implement_to_gpio!(Port::E, Pin::P11, gpioe::PE11);
implement_to_gpio!(Port::E, Pin::P12, gpioe::PE12);
implement_to_gpio!(Port::E, Pin::P13, gpioe::PE13);
implement_to_gpio!(Port::E, Pin::P14, gpioe::PE14);
implement_to_gpio!(Port::E, Pin::P15, gpioe::PE15);
