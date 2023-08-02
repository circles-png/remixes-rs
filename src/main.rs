#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#[warn(clippy::nursery)]
#[warn(clippy::pedantic)]
#[allow(clippy::cast_possible_truncation)]
mod millis;

use arduino_hal::{default_serial, pins, prelude::_void_ResultVoidExt, Peripherals};
use avr_device::interrupt::enable;
use millis::{init, millis};
use ufmt::uwriteln;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Print out panic location
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

    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

fn button() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut led = pins.d11.into_output();
    let button = pins.d7.into_floating_input();

    loop {
        if button.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}

fn button_reverse() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut led = pins.d11.into_output();
    let button = pins.d7.into_floating_input();

    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}

fn button_three_seconds() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);
    let mut serial = default_serial!(peripherals, pins, 57600);

    let mut led = pins.d11.into_output();
    let button = pins.d7.into_floating_input();
    init(&peripherals.TC0);
    let mut last_off = millis();

    loop {
        if button.is_high() {
            led.set_high();
            last_off = millis();
        } else if millis() - last_off > 3000 {
            led.set_low();
        }

        uwriteln!(&mut serial, "millis: {}, last_off: {}", millis(), last_off).void_unwrap();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    button_three_seconds()
}
