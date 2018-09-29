#![no_std]
#![no_main]
extern crate arduino_leonardo;
extern crate atmega32u4;
extern crate atmega32u4_hal;

use arduino_leonardo::prelude::*;

#[no_mangle]
pub extern fn main() {
    let dp = atmega32u4::Peripherals::take().unwrap();

    let mut delay = arduino_leonardo::Delay::new();

    let mut pwm4 = atmega32u4_hal::timer::Timer4Pwm::new(dp.TIMER4);

    let mut portc = dp.PORTC.split();
    let mut pin = portc.pc7.into_output(&mut portc.ddr).into_pwm(&mut pwm4);

    let mut up = true;
    loop {
        for i in 0..0xff {
            let index = if up { i } else { 0xfe - i };
            let duty_cycle = (index as u16).pow(2) / 255 + 1;
            pin.set_duty_cycle(duty_cycle as u8);
            delay.delay_us(2000u16);
        }
        up = !up;
    }
}
