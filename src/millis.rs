use arduino_hal::pac::TC0;
use avr_device::interrupt::enable;
use avr_device::{
    interrupt,
    interrupt::{free, Mutex},
};
use core::cell::Cell;
pub const PRESCALER: u32 = 1024;
pub const TIMER_COUNTS: u32 = 125;
pub const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16_000;
pub static MILLIS_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
pub fn init(tc0: &TC0) {
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(TIMER_COUNTS as u8));
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => unreachable!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });

    unsafe { enable() }
}

#[interrupt(atmega328p)]
fn TIMER0_COMPA() {
    free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    });
}

pub fn millis() -> u32 {
    free(|cs| MILLIS_COUNTER.borrow(cs).get())
}
