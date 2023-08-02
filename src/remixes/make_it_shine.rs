use arduino_hal::{delay_ms, pins, Peripherals};

pub fn make_it_shine_original() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut pins = [
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d11.into_output().downgrade(),
        pins.d12.into_output().downgrade(),
    ];

    loop {
        for pin in &mut pins {
            pin.set_high();
        }
        delay_ms(1000);
        for pin in &mut pins {
            pin.set_low();
        }
        delay_ms(1000);
    }
}

pub fn make_it_shine_double() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut pin_groups = [
        [
            pins.d9.into_output().downgrade(),
            pins.d11.into_output().downgrade(),
        ],
        [
            pins.d10.into_output().downgrade(),
            pins.d12.into_output().downgrade(),
        ],
    ];

    loop {
        for pin in &mut pin_groups[0] {
            pin.set_high();
        }
        for pin in &mut pin_groups[1] {
            pin.set_low();
        }
        delay_ms(1000);
        for pin in &mut pin_groups[0] {
            pin.set_low();
        }
        for pin in &mut pin_groups[1] {
            pin.set_high();
        }
        delay_ms(1000);
    }
}

pub fn make_it_shine_chase() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut pins = [
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d11.into_output().downgrade(),
        pins.d12.into_output().downgrade(),
    ];

    let mut on = 0;

    loop {
        for pin in &mut pins {
            pin.set_low();
        }
        pins[on].set_high();
        on = (on + 1) % pins.len();
        delay_ms(100);
    }
}

pub fn make_it_shine_binary() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut pins = [
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d11.into_output().downgrade(),
        pins.d12.into_output().downgrade(),
    ];

    let mut number = 0;

    loop {
        number = (number + 1) % (1 << pins.len());
        for (index, pin) in pins.iter_mut().enumerate() {
            if number & (1 << index) != 0 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    }
}
