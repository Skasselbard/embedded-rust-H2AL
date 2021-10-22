use crate::device::{TimerError, TimerID};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct Hertz(pub u32);
pub trait FromHertz {
    fn from_hertz(frequency: Hertz) -> Self;
}

pub trait ToTimerID {
    fn to_timer_id(&self) -> TimerID;
}
pub trait Timer: ToTimerID {
    fn start(&mut self, interval: Hertz);
    fn cancel(&mut self) -> Result<(), TimerError>;
    fn wait(&mut self) -> Result<(), ()>;
}

#[repr(transparent)]

pub struct GeneralPurposeTimer(pub(crate) &'static mut dyn Timer);

impl Hertz {
    pub const fn hz(frequency: u32) -> Self {
        Self(frequency)
    }
    pub const fn khz(frequency: u32) -> Self {
        Self::hz(1000 * frequency)
    }
    pub const fn mhz(frequency: u32) -> Self {
        Self::khz(1000 * frequency)
    }
    pub const fn ghz(frequency: u32) -> Self {
        Self::mhz(1000 * frequency)
    }
    pub fn to_milliseconds(&self) -> u32 {
        unimplemented!() //TODO: make constant
    }
}

impl PartialEq for GeneralPurposeTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_timer_id() == other.0.to_timer_id()
    }
}
impl Eq for GeneralPurposeTimer {}
impl PartialOrd for GeneralPurposeTimer {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.to_timer_id().partial_cmp(&other.0.to_timer_id())
    }
}
impl Ord for GeneralPurposeTimer {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.to_timer_id().cmp(&other.0.to_timer_id())
    }
}
