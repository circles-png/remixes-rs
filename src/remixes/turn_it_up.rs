use arduino_hal::{
    default_serial, delay_ms, pins,
    prelude::_void_ResultVoidExt,
    simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm},
    Adc, Peripherals,
};
use ufmt::uwriteln;

fn map(value: u16, from_low: u16, from_high: u16, to_low: u16, to_high: u16) -> u16 {
    let from_range = from_high - from_low;
    let to_range = to_high - to_low;
    let value_scaled = value - from_low;
    let value_mapped = value_scaled * to_range / from_range;
    value_mapped + to_low
}

pub fn turn_it_up_original() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);
    let mut serial = default_serial!(peripherals, pins, 115200);
    let mut adc = Adc::new(peripherals.ADC, Default::default());
    let timer = Timer1Pwm::new(peripherals.TC1, Prescaler::Prescale64);
    let mut led = pins.d9.into_output().into_pwm(&timer);
    led.enable();
    let pot = pins.a5.into_analog_input(&mut adc);

    loop {
        let value = pot.analog_read(&mut adc);
        uwriteln!(&mut serial, "{}", value).void_unwrap();
        let value = map(value, 0, 1023, 0, 255) as u8;
        led.set_duty(value);
        delay_ms(25);
    }
}

pub fn turn_it_up_reverse() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);
    let mut serial = default_serial!(peripherals, pins, 115200);
    let mut adc = Adc::new(peripherals.ADC, Default::default());
    let timer = Timer1Pwm::new(peripherals.TC1, Prescaler::Prescale64);
    let mut led = pins.d9.into_output().into_pwm(&timer);
    led.enable();
    let pot = pins.a5.into_analog_input(&mut adc);

    loop {
        let value = pot.analog_read(&mut adc);
        uwriteln!(&mut serial, "{}", value).void_unwrap();
        let value = map(value, 0, 1023, 0, 255) as u8;
        led.set_duty(255 - value);
        delay_ms(25);
    }
}

pub fn turn_it_up_opposites() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);
    let mut adc = Adc::new(peripherals.ADC, Default::default());
    let pot = pins.a5.into_analog_input(&mut adc);
    let mut led_pairs = [
        [
            pins.d9.into_output().downgrade(),
            pins.d12.into_output().downgrade(),
        ],
        [
            pins.d10.into_output().downgrade(),
            pins.d11.into_output().downgrade(),
        ],
    ];

    loop {
        let value = pot.analog_read(&mut adc);
        let value = map(value, 0, 1023, 0, 3) as u8;
        led_pairs.iter_mut().flatten().for_each(|pin| pin.set_low());
        match value {
            0 | 3 => led_pairs[0].iter_mut().for_each(|pin| pin.set_high()),
            1 | 2 => led_pairs[1].iter_mut().for_each(|pin| pin.set_high()),
            _ => unreachable!(),
        }
    }
}
