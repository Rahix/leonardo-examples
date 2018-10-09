//! PWM
//! ===
//!
//! Example of using a pwm pin
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
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    // According to the manual, PC7(D13) is connected to Timer/Counter4
    // If you don't want to look it up, just try supplying a wrong one, rustc will
    // tell you which one you need ;)
    let mut pwm4 = atmega32u4_hal::timer::Timer4Pwm::new(dp.TIMER4);

    // First make the pin an output, then enable the PWM timer
    let mut led = pins.d13.into_output(&mut pins.ddr).into_pwm(&mut pwm4);

    let mut up = true;
    loop {
        for i in 0..0xff {
            // Fade up and down
            let index = if up { i } else { 0xfe - i } as u16;
            // Scale brightness quadratic instead of linearly
            let duty_cycle = ((index * index) >> 8) + 1;
            led.set_duty(duty_cycle as u8);
            delay.delay_us(2000u16);
        }
        up = !up;
    }
}
