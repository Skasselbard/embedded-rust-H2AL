use heapless::spsc::Queue;

use crate::device::{disable_interrupts, enable_interrupts, TimerID};

const EVENT_QUEUE_SIZE: usize = 16;
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Event {
    TimerEvent(TimerID),
}
unsafe impl Sync for Event {}

static mut EVENT_QUEUE: Queue<Event, EVENT_QUEUE_SIZE> = Queue::new();

#[inline]
pub(crate) fn queue_event(e: Event) {
    disable_interrupts();
    unsafe {
        EVENT_QUEUE
            .split()
            .0
            .enqueue(e)
            .expect("EventQueue Overflow")
    };
    enable_interrupts();
}

#[inline]
pub fn next_event() -> Option<Event> {
    unsafe { EVENT_QUEUE.split().1.dequeue() }
}
