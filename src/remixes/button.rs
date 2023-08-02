use crate::millis::{init, millis};
use arduino_hal::{pins, Peripherals};

pub fn button_original() -> ! {
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

pub fn button_reverse() -> ! {
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

pub fn button_three_seconds() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

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
    }
}

pub fn button_toggle() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut led = pins.d11.into_output();
    let button = pins.d7.into_floating_input();

    loop {
        if button.is_high() {
            led.toggle();
            while button.is_high() {}
        }
    }
}
