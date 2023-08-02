#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#[warn(clippy::nursery)]
#[warn(clippy::pedantic)]
#[allow(clippy::cast_possible_truncation)]
mod millis;
mod remixes;

use arduino_hal::prelude::_void_ResultVoidExt;
use core::panic::PanicInfo;
use remixes::button_three_seconds;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .void_unwrap();
    }

    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    button_three_seconds()
}
